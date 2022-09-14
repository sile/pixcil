use super::pixel_canvas::PixelPosition;
use crate::serialize::{Deserialize, Serialize};
use pagurus::{failure::OrFail, Result};
use std::io::{Read, Write};

#[derive(Debug, Default)]
pub struct ConfigModel {
    pub zoom: Zoom,
    pub camera: Camera,
}

impl Serialize for ConfigModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let size = self.serialized_size().or_fail()?;
        let size = u16::try_from(size).or_fail()?;
        size.serialize(writer).or_fail()?;

        self.zoom.serialize(writer).or_fail()?;
        self.camera.serialize(writer).or_fail()?;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Camera(PixelPosition);

impl Camera {
    pub const MIN: Self = Self(PixelPosition::from_xy(-20000, -20000));
    pub const MAX: Self = Self(PixelPosition::from_xy(20000, 20000));

    pub const fn get(self) -> PixelPosition {
        self.0
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

fn clip<T: Ord>(min: T, value: T, max: T) -> T {
    std::cmp::min(std::cmp::max(min, value), max)
}
