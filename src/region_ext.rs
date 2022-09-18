use pagurus::spatial::Region;

pub trait RegionExt: Sized {
    fn without_margin(self, margin: u32) -> Self;
}

impl RegionExt for Region {
    fn without_margin(mut self, margin: u32) -> Self {
        self.position = self.position + margin as i32;
        self.size = self.size - margin * 2;
        self
    }
}
