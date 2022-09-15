use crate::serialize::{Deserialize, Serialize};
use pagurus::Result;
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
