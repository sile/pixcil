use pagurus::spatial::Region;

pub trait RegionExt: Sized {
    fn without_margin(self, margin: u32) -> Self;
    fn consume_y(&mut self, n: u32);
}

impl RegionExt for Region {
    fn without_margin(mut self, margin: u32) -> Self {
        self.position = self.position + margin as i32;
        self.size = self.size - margin * 2;
        self
    }

    fn consume_y(&mut self, n: u32) {
        self.position.y += n as i32;
        self.size.height -= n;
    }
}
