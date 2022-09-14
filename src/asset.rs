use pagurus::Result;

#[derive(Debug)]
pub struct Assets {}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Self {})
    }
}
