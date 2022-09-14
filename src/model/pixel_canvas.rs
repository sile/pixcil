use crate::{
    app::App,
    serialize::{Deserialize, Serialize},
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region},
    Result,
};
use std::{
    io::{Read, Write},
    ops::Add,
};

#[derive(Debug, Default)]
pub struct PixelCanvasModel {}

impl Serialize for PixelCanvasModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Deserialize for PixelCanvasModel {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {})
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
        let center = app.screen_size().to_region().center() - zoom / 2;

        fn offset(pos: i32, center: i32, zoom: i32) -> i16 {
            if center <= pos {
                ((pos - center) / zoom) as i16
            } else {
                ((pos + 1 - center) / zoom - 1) as i16
            }
        }

        Self::from_xy(
            camera.x.saturating_add(offset(screen.x, center.x, zoom)),
            camera.y.saturating_add(offset(screen.y, center.y, zoom)),
        )
    }
}

impl Serialize for PixelPosition {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.y.serialize(writer).or_fail()?;
        self.x.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for PixelPosition {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            y: Deserialize::deserialize(reader).or_fail()?,
            x: Deserialize::deserialize(reader).or_fail()?,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelRegion {
    pub start: PixelPosition,
    pub end: PixelPosition,
}

impl PixelRegion {
    pub fn from_screen_region(app: &App, screen: Region) -> Self {
        let start = PixelPosition::from_screen_position(app, screen.start());
        let end = PixelPosition::from_screen_position(app, screen.end() - 1) + 1;
        Self { start, end }
    }
}
