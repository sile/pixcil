use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use orfail::{Failure, OrFail, Result};
use pagurus::image::Rgba;
use pagurus::spatial::Position;
use std::time::Duration;
use std::{
    collections::VecDeque,
    io::{Read, Write},
};

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

impl Serialize for u32 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<BigEndian>(*self).or_fail()
    }
}

impl Serialize for i32 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32::<BigEndian>(*self).or_fail()
    }
}

impl Serialize for u64 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u64::<BigEndian>(*self).or_fail()
    }
}

impl Serialize for usize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let n = u32::try_from(*self).or_fail()?;
        writer.write_u32::<BigEndian>(n).or_fail()
    }
}

impl Serialize for Duration {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_secs().serialize(writer).or_fail()
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Some(value) => {
                1u8.serialize(writer).or_fail()?;
                value.serialize(writer).or_fail()
            }
            None => 0u8.serialize(writer).or_fail(),
        }
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

    fn deserialize_or<R: Read>(reader: &mut R, default: Self) -> Result<Self>
    where
        Self: Default,
    {
        let mut buf = [0];
        let n = reader.read(&mut buf).or_fail()?;
        if n == 0 {
            Ok(default)
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

impl Deserialize for u32 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u32::<BigEndian>().or_fail()
    }
}

impl Deserialize for i32 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_i32::<BigEndian>().or_fail()
    }
}

impl Deserialize for u64 {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u64::<BigEndian>().or_fail()
    }
}

impl Deserialize for usize {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = reader.read_u32::<BigEndian>().or_fail()?;
        usize::try_from(n).or_fail()
    }
}

impl Deserialize for Duration {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let secs = u64::deserialize(reader).or_fail()?;
        Ok(Duration::from_secs(secs))
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = u8::deserialize(reader).or_fail()?;
        match n {
            0 => Ok(None),
            1 => Ok(Some(T::deserialize(reader).or_fail()?)),
            _ => Err(Failure::new(format!(
                "expected 0 (Some(_)) or 1 (None), but got {n}"
            ))),
        }
    }
}

impl Serialize for Rgba {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.r.serialize(writer).or_fail()?;
        self.g.serialize(writer).or_fail()?;
        self.b.serialize(writer).or_fail()?;
        self.a.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Rgba {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            r: u8::deserialize(reader).or_fail()?,
            g: u8::deserialize(reader).or_fail()?,
            b: u8::deserialize(reader).or_fail()?,
            a: u8::deserialize(reader).or_fail()?,
        })
    }
}

impl<T: Serialize> Serialize for VecDeque<T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.len().serialize(writer).or_fail()?;
        for item in self {
            item.serialize(writer).or_fail()?;
        }
        Ok(())
    }
}

impl<T: Deserialize> Deserialize for VecDeque<T> {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = usize::deserialize(reader).or_fail()?;
        (0..n).map(|_| T::deserialize(reader).or_fail()).collect()
    }
}

impl Serialize for bool {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(*self as u8).or_fail()
    }
}

impl Deserialize for bool {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_u8().or_fail()? == 1)
    }
}

impl Serialize for Position {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.x.serialize(writer).or_fail()?;
        self.y.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Position {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self::from_xy(
            Deserialize::deserialize(reader).or_fail()?,
            Deserialize::deserialize(reader).or_fail()?,
        ))
    }
}
