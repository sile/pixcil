use crate::{
    pixel::{Pixel, PixelPosition, PixelRegion},
    serialize::{Deserialize, Serialize},
};
use pagurus::{failure::OrFail, Result};
use pagurus_game_std::color::Rgba;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{Read, Write},
};

#[derive(Debug, Default)]
pub struct PixelCanvasModel {
    command_log: VecDeque<PixelCanvasCommand>,
    command_log_tail: usize,
    pixels: Pixels,

    // The following fields are not serialized / deserialized
    dirty_positions: BTreeSet<PixelPosition>,
}

impl PixelCanvasModel {
    pub fn draw_pixels(&mut self, pixels: impl Iterator<Item = Pixel>) -> Result<()> {
        let mut command = PixelCanvasCommand::default();
        command.draw = pixels.collect();
        command.draw.sort_by_key(|x| x.position);
        for pixel in &mut command.draw {
            if let Some(color) = self.pixels.get_pixel(pixel.position) {
                pixel.color = pixel.color.alpha_blend(color);
                command.erase.push(Pixel::new(pixel.position, color));
            }
        }
        self.command_log.truncate(self.command_log_tail);
        self.command_log.push_back(command);
        self.redo_command().or_fail()?;
        Ok(())
    }

    pub fn erase_pixels(&mut self, pixels: impl Iterator<Item = PixelPosition>) -> Result<()> {
        let mut command = PixelCanvasCommand::default();
        for position in pixels {
            if let Some(color) = self.pixels.get_pixel(position) {
                command.erase.push(Pixel::new(position, color));
            }
        }
        command.erase.sort_by_key(|x| x.position);
        self.command_log.truncate(self.command_log_tail);
        self.command_log.push_back(command);
        self.redo_command().or_fail()?;
        Ok(())
    }

    pub fn move_pixels(
        &mut self,
        pixels: impl Iterator<Item = PixelPosition>,
        delta: PixelPosition,
    ) -> Result<()> {
        let mut command = PixelCanvasCommand::default();
        for src in pixels {
            let dst = src + delta;
            if let Some(color) = self.pixels.get_pixel(src) {
                command.erase.push(Pixel::new(src, color));
                let color = if let Some(old_color) = self.pixels.get_pixel(dst) {
                    command.erase.push(Pixel::new(dst, old_color));
                    color.alpha_blend(old_color)
                } else {
                    color
                };
                command.draw.push(Pixel::new(dst, color));
            }
        }
        command.draw.sort_by_key(|x| x.position);
        command.erase.sort_by_key(|x| x.position);
        command.erase.dedup();

        self.command_log.truncate(self.command_log_tail);
        self.command_log.push_back(command);
        self.redo_command().or_fail()?;

        Ok(())
    }

    pub fn get_pixels(&self, region: PixelRegion) -> impl '_ + Iterator<Item = Pixel> {
        self.pixels.get_pixels(region)
    }

    pub fn get_pixel(&self, position: PixelPosition) -> Option<Rgba> {
        self.pixels.get_pixel(position)
    }

    pub fn undo_command(&mut self) -> Result<()> {
        if let Some(i) = self.command_log_tail.checked_sub(1) {
            let command = &self.command_log[i];
            for &pixel in &command.draw {
                self.pixels.erase_pixel(pixel).or_fail()?;
                self.dirty_positions.insert(pixel.position);
            }
            for &pixel in &command.erase {
                self.pixels.draw_pixel(pixel).or_fail()?;
                self.dirty_positions.insert(pixel.position);
            }
            self.command_log_tail = i;
        }
        Ok(())
    }

    pub fn redo_command(&mut self) -> Result<()> {
        if let Some(command) = self.command_log.get(self.command_log_tail) {
            for &pixel in &command.erase {
                self.pixels.erase_pixel(pixel).or_fail()?;
                self.dirty_positions.insert(pixel.position);
            }
            for &pixel in &command.draw {
                self.pixels.draw_pixel(pixel).or_fail()?;
                self.dirty_positions.insert(pixel.position);
            }
            self.command_log_tail += 1;
        }
        Ok(())
    }

    pub fn forget_oldest_command(&mut self) {
        if self.command_log_tail > 0 {
            self.command_log.pop_front();
            self.command_log_tail -= 1;
        }
    }

    pub fn command_log_tail(&self) -> usize {
        self.command_log_tail
    }

    pub fn command_log(&self) -> &VecDeque<PixelCanvasCommand> {
        &self.command_log
    }

    pub fn take_dirty_positions(&mut self) -> BTreeSet<PixelPosition> {
        std::mem::take(&mut self.dirty_positions)
    }

    pub fn dirty_positions(&self) -> &BTreeSet<PixelPosition> {
        &self.dirty_positions
    }
}

impl Serialize for PixelCanvasModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut writer = libflate::deflate::Encoder::new(writer);
        self.command_log.serialize(&mut writer).or_fail()?;
        self.command_log_tail.serialize(&mut writer).or_fail()?;
        self.pixels.serialize(&mut writer).or_fail()?;
        writer.finish().into_result().or_fail()?;
        Ok(())
    }
}

impl Deserialize for PixelCanvasModel {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut reader = libflate::deflate::Decoder::new(reader);
        Ok(Self {
            command_log: Deserialize::deserialize(&mut reader).or_fail()?,
            command_log_tail: Deserialize::deserialize(&mut reader).or_fail()?,
            pixels: Deserialize::deserialize(&mut reader).or_fail()?,
            dirty_positions: Default::default(),
        })
    }
}

#[derive(Debug, Default)]
pub struct PixelCanvasCommand {
    pub erase: Vec<Pixel>,
    pub draw: Vec<Pixel>,
}

impl Serialize for PixelCanvasCommand {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.erase.len().serialize(writer).or_fail()?;
        self.draw.len().serialize(writer).or_fail()?;
        serialize_positions(writer, || {
            self.erase
                .iter()
                .chain(self.draw.iter())
                .map(|pixel| pixel.position)
        })
        .or_fail()?;
        for pixel in self.erase.iter().chain(self.draw.iter()) {
            pixel.color.serialize(writer).or_fail()?;
        }
        Ok(())
    }
}

impl Deserialize for PixelCanvasCommand {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let erase_len = usize::deserialize(reader).or_fail()?;
        let draw_len = usize::deserialize(reader).or_fail()?;

        let positions = deserialize_positions(reader, erase_len + draw_len)
            .or_fail()?
            .collect::<Vec<_>>();
        let erase = positions[..erase_len]
            .iter()
            .copied()
            .map(|pos| Ok(Pixel::new(pos, Rgba::deserialize(reader).or_fail()?)))
            .collect::<Result<Vec<_>>>()?;
        let draw = positions[erase_len..]
            .iter()
            .copied()
            .map(|pos| Ok(Pixel::new(pos, Rgba::deserialize(reader).or_fail()?)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { erase, draw })
    }
}

#[derive(Debug, Default)]
struct Pixels(BTreeMap<PixelPosition, Rgba>);

impl Pixels {
    fn get_pixels(&self, region: PixelRegion) -> impl '_ + Iterator<Item = Pixel> {
        (region.start.y..region.end.y).flat_map(move |y| {
            let start = PixelPosition::from_xy(region.start.x, y);
            let end = PixelPosition::from_xy(region.end.x, y);
            self.0
                .range(start..end)
                .map(|(pos, color)| Pixel::new(*pos, *color))
        })
    }

    fn get_pixel(&self, position: PixelPosition) -> Option<Rgba> {
        self.0.get(&position).copied()
    }

    fn draw_pixel(&mut self, pixel: Pixel) -> Result<()> {
        let prev = self.0.insert(pixel.position, pixel.color);
        prev.is_none().or_fail()
    }

    fn erase_pixel(&mut self, pixel: Pixel) -> Result<()> {
        let prev = self.0.remove(&pixel.position);
        (prev == Some(pixel.color)).or_fail()
    }
}

impl Serialize for Pixels {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.len().serialize(writer).or_fail()?;
        serialize_positions(writer, || self.0.keys().copied()).or_fail()?;

        for color in self.0.values() {
            color.serialize(writer).or_fail()?;
        }

        Ok(())
    }
}

impl Deserialize for Pixels {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = usize::deserialize(reader).or_fail()?;
        deserialize_positions(reader, n)
            .or_fail()?
            .map(|pos| Ok((pos, Rgba::deserialize(reader).or_fail()?)))
            .collect::<Result<_>>()
            .map(Self)
    }
}

fn serialize_positions<W: Write, F, I>(writer: &mut W, f: F) -> Result<()>
where
    F: Fn() -> I,
    I: Iterator<Item = PixelPosition>,
{
    let mut prev_y = 0;
    for pos in f() {
        (pos.y - prev_y).serialize(writer).or_fail()?;
        prev_y = pos.y;
    }

    let mut prev_x = 0;
    for pos in f() {
        (pos.x - prev_x).serialize(writer).or_fail()?;
        prev_x = pos.x;
    }
    Ok(())
}

fn deserialize_positions<R: Read>(
    reader: &mut R,
    size: usize,
) -> Result<impl Iterator<Item = PixelPosition>> {
    let mut y = 0;
    let mut ys = Vec::with_capacity(size);
    for _ in 0..size {
        let delta = i16::deserialize(reader).or_fail()?;
        y = y + delta;
        ys.push(y);
    }

    let mut positions = Vec::with_capacity(size);
    let mut x = 0;
    for y in ys {
        let delta = i16::deserialize(reader).or_fail()?;
        x = x + delta;
        positions.push(PixelPosition::from_xy(x, y));
    }

    Ok(positions.into_iter())
}
