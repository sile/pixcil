use crate::serialize::{Deserialize, Serialize};
use pagurus::{failure::OrFail, Result, System};
use std::{
    io::{Read, Write},
    time::Duration,
};

#[derive(Debug, Default, Clone)]
pub struct AttributesModel {
    pub created_time: Option<Duration>,
    pub updated_time: Option<Duration>,
}

impl AttributesModel {
    pub fn update_time<S: System>(&mut self, system: &mut S) {
        let now = system.clock_unix_time();
        if self.created_time.is_none() {
            self.created_time = Some(now);
        }
        self.updated_time = Some(now);
    }
}

impl Serialize for AttributesModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.created_time.serialize(writer).or_fail()?;
        self.updated_time.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for AttributesModel {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            created_time: Deserialize::deserialize(reader).or_fail()?,
            updated_time: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}
