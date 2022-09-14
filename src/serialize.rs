use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use pagurus::{failure::OrFail, Result};
use std::io::{Read, Write};

pub trait Serialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn serialized_size(&self) -> Result<usize> {
        struct Counter(usize);

        impl Write for Counter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.0 += buf.len();
                Ok(buf.len())
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut counter = Counter(0);
        self.serialize(&mut counter).or_fail()?;
        Ok(counter.0)
    }
}

impl<const N: usize> Serialize for [u8; N] {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self).or_fail()
    }
}

impl Serialize for u8 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(*self).or_fail()
    }
}

impl Serialize for u16 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16::<BigEndian>(*self).or_fail()
    }
}

impl Serialize for i16 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i16::<BigEndian>(*self).or_fail()
    }
}

impl Serialize for usize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let n = u32::try_from(*self).or_fail()?;
        writer.write_u32::<BigEndian>(n).or_fail()
    }
}

pub trait Deserialize: Sized {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>;

    fn deserialize_or_default<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Default,
    {
        let mut buf = [0];
        let n = reader.read(&mut buf).or_fail()?;
        if n == 0 {
            Ok(Self::default())
        } else {
            Self::deserialize(&mut buf.chain(reader)).or_fail()
        }
    }
}

impl<const N: usize> Deserialize for [u8; N] {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; N];
        reader.read_exact(&mut buf).or_fail()?;
        Ok(buf)
    }
}

impl Deserialize for u8 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u8().or_fail()
    }
}

impl Deserialize for u16 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u16::<BigEndian>().or_fail()
    }
}

impl Deserialize for i16 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_i16::<BigEndian>().or_fail()
    }
}

impl Deserialize for usize {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = reader.read_u32::<BigEndian>().or_fail()?;
        usize::try_from(n).or_fail()
    }
}
