use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use pagurus::{failure::OrFail, Result};
use std::io::{Read, Write};

pub trait Serialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;
}

impl<const N: usize> Serialize for [u8; N] {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self).or_fail()
    }
}

impl Serialize for u16 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16::<BigEndian>(*self).or_fail()
    }
}

pub trait Deserialize: Sized {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>;
}

impl<const N: usize> Deserialize for [u8; N] {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; N];
        reader.read_exact(&mut buf).or_fail()?;
        Ok(buf)
    }
}

impl Deserialize for u16 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u16::<BigEndian>().or_fail()
    }
}
