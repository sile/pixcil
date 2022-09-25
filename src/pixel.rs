use crate::{
    app::App,
    serialize::{Deserialize, Serialize},
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::color::Rgba;
use std::{
    io::{Read, Write},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pixel {
    pub position: PixelPosition,
    pub color: Rgba,
}

impl Pixel {
    pub const fn new(position: PixelPosition, color: Rgba) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PixelPosition {
    pub y: i16,
    pub x: i16,
}

impl PixelPosition {
    pub const fn from_xy(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn from_screen_position(app: &App, screen: Position) -> Self {
        let zoom = i32::from(app.models().config.zoom.get());
        let camera = app.models().config.camera.get();
        let center = (app.screen_size().to_region().center() - zoom / 2) - camera;

        fn offset(pos: i32, center: i32, zoom: i32) -> i16 {
            if center <= pos {
                ((pos - center) / zoom) as i16
            } else {
                ((pos + 1 - center) / zoom - 1) as i16
            }
        }

        Self::from_xy(
            offset(screen.x, center.x, zoom),
            offset(screen.y, center.y, zoom),
        )
    }

    pub fn to_screen_position(self, app: &App) -> Position {
        let zoom = app.models().config.zoom.get();
        let camera = app.models().config.camera.get();
        let center = (app.screen_size().to_region().center() - i32::from(zoom) / 2) - camera;
        Position::from_xy(i32::from(self.x), i32::from(self.y)) * u32::from(zoom) + center
    }

    pub fn to_screen_region(self, app: &App) -> Region {
        let zoom = app.models().config.zoom.get();
        let position = self.to_screen_position(app);
        let size = Size::square(u32::from(zoom));
        Region::new(position, size)
    }

    pub fn move_x(mut self, delta: i16) -> Self {
        self.x += delta;
        self
    }

    pub fn move_y(mut self, delta: i16) -> Self {
        self.y += delta;
        self
    }
}

impl Serialize for PixelPosition {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.x.serialize(writer).or_fail()?;
        self.y.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for PixelPosition {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            x: Deserialize::deserialize(reader).or_fail()?,
            y: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}

impl Add<i16> for PixelPosition {
    type Output = Self;

    fn add(mut self, rhs: i16) -> Self::Output {
        self.x += rhs;
        self.y += rhs;
        self
    }
}

impl Add for PixelPosition {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl Sub for PixelPosition {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelRegion {
    pub start: PixelPosition,
    pub end: PixelPosition,
}

impl PixelRegion {
    pub const fn new(start: PixelPosition, end: PixelPosition) -> Self {
        Self { start, end }
    }

    pub fn from_position_and_size(position: PixelPosition, size: PixelSize) -> Self {
        let mut end = position;
        end.x += size.width as i16;
        end.y += size.height as i16;
        Self::new(position, end)
    }

    pub fn from_positions(positions: impl Iterator<Item = PixelPosition>) -> Self {
        let mut start = PixelPosition::from_xy(i16::MAX, i16::MAX);
        let mut end = PixelPosition::from_xy(i16::MIN, i16::MIN);
        let mut empty = true;
        for p in positions {
            start.x = std::cmp::min(start.x, p.x);
            start.y = std::cmp::min(start.y, p.y);
            end.x = std::cmp::max(end.x, p.x + 1);
            end.y = std::cmp::max(end.y, p.y + 1);
            empty = false;
        }
        if empty {
            Self::default()
        } else {
            Self { start, end }
        }
    }

    pub fn from_screen_region(app: &App, screen: Region) -> Self {
        let start = PixelPosition::from_screen_position(app, screen.start());
        let end = PixelPosition::from_screen_position(app, screen.end() - 1) + 1;
        Self { start, end }
    }

    pub fn to_screen_region(self, app: &App) -> Region {
        Region::from_positions(
            self.start.to_screen_position(app),
            self.end.to_screen_position(app),
        )
    }

    pub fn size(self) -> PixelSize {
        PixelSize::from_wh(
            std::cmp::max(0, self.end.x - self.start.x) as u16,
            std::cmp::max(0, self.end.y - self.start.y) as u16,
        )
    }

    pub fn pixels(self) -> impl Iterator<Item = PixelPosition> {
        (self.start.y..self.end.y).flat_map(move |y| {
            (self.start.x..self.end.x).map(move |x| PixelPosition::from_xy(x, y))
        })
    }

    pub fn edges(self) -> impl Iterator<Item = PixelPosition> {
        self.pixels().filter(move |p| {
            p.x == self.start.x
                || p.x == self.end.x - 1
                || p.y == self.start.y
                || p.y == self.end.y - 1
        })
    }

    pub fn move_y(self, delta: i16) -> Self {
        let size = self.size();
        let mut position = self.start;
        position.y += delta;
        Self::from_position_and_size(position, size)
    }

    pub fn move_x(self, delta: i16) -> Self {
        let size = self.size();
        let mut position = self.start;
        position.x += delta;
        Self::from_position_and_size(position, size)
    }

    // TODO: refactor
    pub fn set_width(&mut self, w: u16) {
        self.end.x = self.start.x + w as i16;
    }

    pub fn contains(self, position: PixelPosition) -> bool {
        self.start.x <= position.x
            && position.x < self.end.x
            && self.start.y <= position.y
            && position.y < self.end.y
    }

    // TODO: remove
    pub fn center(self) -> PixelPosition {
        let mut size = self.size();
        size.width /= 2;
        size.height /= 2;

        let mut position = self.start;
        position.x += size.width as i16;
        position.y += size.height as i16;

        position
    }
}

impl Serialize for PixelRegion {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.start.serialize(writer).or_fail()?;
        self.end.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for PixelRegion {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            start: Deserialize::deserialize(reader).or_fail()?,
            end: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelSize {
    pub width: u16,
    pub height: u16,
}

impl PixelSize {
    pub const fn from_wh(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub const fn square(size: u16) -> Self {
        Self::from_wh(size, size)
    }
}

impl Serialize for PixelSize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.width.serialize(writer).or_fail()?;
        self.height.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for PixelSize {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            width: u16::deserialize(reader).or_fail()?,
            height: u16::deserialize(reader).or_fail()?,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelLine {
    pub start: PixelPosition,
    pub finish: PixelPosition,
}

impl PixelLine {
    pub const fn new(start: PixelPosition, finish: PixelPosition) -> Self {
        Self { start, finish }
    }

    pub fn pixels(self) -> impl Iterator<Item = PixelPosition> {
        let mut pixels = Vec::new();

        let mut start = self.start;
        let mut finish = self.finish;
        let size = PixelSize::from_wh(
            (finish.x - start.x).abs() as u16,
            (finish.y - start.y).abs() as u16,
        );
        if size.width < size.height {
            if finish.y < start.y {
                std::mem::swap(&mut start, &mut finish);
            }

            let mut delta_x = if size.height == 0 {
                0.0
            } else {
                size.width as f64 / size.height as f64
            };
            let mut x = start.x as f64;
            if finish.x < start.x {
                delta_x = -delta_x;
            }

            for y in start.y..=finish.y {
                pixels.push(PixelPosition::from_xy(x.round() as i16, y));
                x += delta_x;
            }
        } else {
            if finish.x < start.x {
                std::mem::swap(&mut start, &mut finish);
            }

            let mut delta_y = if size.width == 0 {
                0.0
            } else {
                size.height as f64 / size.width as f64
            };
            let mut y = start.y as f64;
            if finish.y < start.y {
                delta_y = -delta_y;
            }

            for x in start.x..=finish.x {
                pixels.push(PixelPosition::from_xy(x, y.round() as i16));
                y += delta_y;
            }
        }

        pixels.into_iter()
    }
}
