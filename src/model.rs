use crate::serialize::{Deserialize, Serialize};
use pagurus::{failure::OrFail, Result};
use png::chunk::ChunkType;
use std::io::{Read, Write};

pub trait Model: Default + Serialize + Deserialize {}

pub const PNG_CHUNK_TYPE: ChunkType = ChunkType(*b"sile");
pub const MAGIC_NUMBER: [u8; 6] = *b"PIXCIL";
pub const FORMAT_VERSION: u16 = 0;

#[derive(Debug, Default)]
pub struct Models {}

impl Model for Models {}

impl Serialize for Models {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        MAGIC_NUMBER.serialize(writer).or_fail()?;
        FORMAT_VERSION.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Models {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let magic_number: [u8; 6] = Deserialize::deserialize(reader).or_fail()?;
        (magic_number == MAGIC_NUMBER).or_fail()?;

        let version: u16 = Deserialize::deserialize(reader).or_fail()?;
        (version == FORMAT_VERSION).or_fail()?;

        Ok(Self {})
    }
}
