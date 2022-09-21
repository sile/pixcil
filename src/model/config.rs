use crate::{
    pixel::{PixelPosition, PixelRegion, PixelSize},
    serialize::{Deserialize, Serialize},
};
use pagurus::{failure::OrFail, Result};
use pagurus_game_std::color::Rgba;
use std::io::{Read, Write};

#[derive(Debug, Default)]
pub struct ConfigModel {
    pub zoom: Zoom,
    pub camera: Camera,
    pub minimum_pixel_size: MinimumPixelSize,
    pub max_undos: MaxUndos,
    pub color: DrawingColor,
    pub frame: FrameRegion,
    pub frame_preview: FramePreview,
}

impl Serialize for ConfigModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let size = self.serialized_size().or_fail()?;
        let size = u16::try_from(size).or_fail()?;
        size.serialize(writer).or_fail()?;

        self.zoom.serialize(writer).or_fail()?;
        self.camera.serialize(writer).or_fail()?;
        self.minimum_pixel_size.serialize(writer).or_fail()?;
        self.max_undos.serialize(writer).or_fail()?;
        self.color.serialize(writer).or_fail()?;
        self.frame.serialize(writer).or_fail()?;
        self.frame_preview.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for ConfigModel {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let size = u16::deserialize(reader).or_fail()?;
        let mut reader = reader.take(u64::from(size));

        let this = Self {
            zoom: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            camera: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            minimum_pixel_size: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            max_undos: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            color: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            frame: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
            frame_preview: Deserialize::deserialize_or_default(&mut reader).or_fail()?,
        };

        // Ignore unknown fields.
        for _ in reader.bytes() {}

        Ok(this)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Zoom(u8);

impl Zoom {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(50);

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn is_min(self) -> bool {
        self.0 == Self::MIN.0
    }

    pub const fn is_max(self) -> bool {
        self.0 == Self::MAX.0
    }

    pub fn decrement(&mut self) {
        self.0 = std::cmp::max(Self::MIN.0, self.0 - 1);
    }

    pub fn increment(&mut self) {
        self.0 = std::cmp::min(Self::MAX.0, self.0 + 1);
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom(8)
    }
}

impl Serialize for Zoom {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for Zoom {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = u8::deserialize(reader).map(Self).or_fail()?;
        Ok(clip(Self::MIN, n, Self::MAX))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Camera(PixelPosition);

impl Camera {
    pub const MIN: Self = Self(PixelPosition::from_xy(-20000, -20000));
    pub const MAX: Self = Self(PixelPosition::from_xy(20000, 20000));

    pub const fn get(self) -> PixelPosition {
        self.0
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self(PixelPosition::from_xy(32, 32))
    }
}

impl Serialize for Camera {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for Camera {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let p = PixelPosition::deserialize(reader).or_fail()?;
        let x = clip(Self::MIN.0.x, p.x, Self::MAX.0.x);
        let y = clip(Self::MIN.0.y, p.y, Self::MAX.0.y);
        Ok(Self(PixelPosition::from_xy(x, y)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinimumPixelSize(PixelSize);

impl MinimumPixelSize {
    pub const MIN: Self = Self(PixelSize::square(1));
    pub const MAX: Self = Self(PixelSize::square(1000));

    pub const fn get(self) -> PixelSize {
        self.0
    }

    pub fn set(&mut self, size: PixelSize) {
        self.0.width = clip(Self::MIN.0.width, size.width, Self::MAX.0.width);
        self.0.height = clip(Self::MIN.0.height, size.height, Self::MAX.0.height);
    }

    pub fn normalize(self, mut position: PixelPosition) -> PixelPosition {
        let w = self.0.width as i16;
        if position.x >= 0 {
            position.x /= w;
        } else {
            position.x = (position.x - (w - 1)) / w;
        }

        let h = self.0.height as i16;
        if position.y >= 0 {
            position.y /= h;
        } else {
            position.y = (position.y - (h - 1)) / h;
        }
        position
    }

    pub fn denormalize(self, mut position: PixelPosition) -> PixelPosition {
        position.x *= self.0.width as i16;
        position.y *= self.0.height as i16;
        position
    }

    pub fn denormalize_to_region(self, position: PixelPosition) -> PixelRegion {
        let start = self.denormalize(position);
        let mut end = start;
        end.x += self.0.width as i16;
        end.y += self.0.height as i16;
        PixelRegion { start, end }
    }

    pub fn to_region(self, position: PixelPosition) -> PixelRegion {
        self.denormalize_to_region(self.normalize(position))
    }
}

impl Default for MinimumPixelSize {
    fn default() -> Self {
        Self(PixelSize::square(1))
    }
}

impl Serialize for MinimumPixelSize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for MinimumPixelSize {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut size = PixelSize::deserialize(reader).or_fail()?;
        size.width = clip(Self::MIN.0.width, size.width, Self::MAX.0.width);
        size.height = clip(Self::MIN.0.height, size.height, Self::MAX.0.height);
        Ok(Self(size))
    }
}

fn clip<T: Ord>(min: T, value: T, max: T) -> T {
    std::cmp::min(std::cmp::max(min, value), max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawingColor(Rgba);

impl DrawingColor {
    pub const fn get(self) -> Rgba {
        self.0
    }
}

impl Default for DrawingColor {
    fn default() -> Self {
        Self(Rgba::new(0, 0, 0, 255))
    }
}

impl Serialize for DrawingColor {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for DrawingColor {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Rgba::deserialize(reader).map(Self).or_fail()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameRegion(PixelRegion);

impl FrameRegion {
    pub const fn get(self) -> PixelRegion {
        self.0
    }

    pub fn set_width(&mut self, width: u16) {
        let mut size = self.0.size();
        size.width = width;
        self.0 = PixelRegion::from_position_and_size(self.0.start, size);
    }

    pub fn set_height(&mut self, height: u16) {
        let mut size = self.0.size();
        size.height = height;
        self.0 = PixelRegion::from_position_and_size(self.0.start, size);
    }
}

impl Default for FrameRegion {
    fn default() -> Self {
        Self(PixelRegion::new(
            PixelPosition::from_xy(0, 0),
            PixelPosition::from_xy(64, 64),
        ))
    }
}

impl Serialize for FrameRegion {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for FrameRegion {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        PixelRegion::deserialize(reader).map(Self).or_fail()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FramePreview(bool);

impl FramePreview {
    pub const fn get(self) -> bool {
        self.0
    }

    pub fn set(&mut self, on: bool) {
        self.0 = on;
    }
}

impl Default for FramePreview {
    fn default() -> Self {
        Self(true)
    }
}

impl Serialize for FramePreview {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for FramePreview {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        bool::deserialize(reader).map(Self).or_fail()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaxUndos(u32);

impl MaxUndos {
    pub const fn get(self) -> u32 {
        self.0
    }

    pub fn set(&mut self, n: u32) {
        self.0 = n;
    }
}

impl Default for MaxUndos {
    fn default() -> Self {
        Self(100)
    }
}

impl Serialize for MaxUndos {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for MaxUndos {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        u32::deserialize(reader).map(Self).or_fail()
    }
}
