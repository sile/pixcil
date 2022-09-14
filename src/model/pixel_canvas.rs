use crate::serialize::{Deserialize, Serialize};
use pagurus::{failure::OrFail, Result};
use std::io::{Read, Write};

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
