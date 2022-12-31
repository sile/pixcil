use crate::{
    pixel::{Pixel, PixelPosition, PixelRegion},
    serialize::{Deserialize, Serialize},
};
use pagurus::image::Rgba;
use pagurus::{failure::OrFail, Result};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{Read, Write},
};

use super::config::{ConfigModel, FrameRegion, Layer};

#[derive(Debug, Default)]
pub struct PixelCanvasModel {
    command_log: VecDeque<PixelCanvasCommand>,
    command_log_tail: usize,
    pixels: Pixels,

    // The following fields are not serialized / deserialized
    dirty_positions: BTreeSet<PixelPosition>,
    state_version: i64,
}

impl PixelCanvasModel {
    pub fn state_version(&self) -> i64 {
        self.state_version
    }

    pub fn draw_pixels(
        &mut self,
        config: &ConfigModel,
        pixels: impl Iterator<Item = Pixel>,
    ) -> Result<()> {
        let mut command = PixelCanvasCommand::new();
        command.draw = pixels.collect();
        command.draw.sort_by_key(|x| x.position);
        for pixel in &mut command.draw {
            if let Some(color) = self.pixels.get_pixel(pixel.position) {
                pixel.color = pixel.color.alpha_blend(color);
                command.erase.push(Pixel::new(pixel.position, color));
            }
        }
        self.apply_command(config, command).or_fail()?;
        Ok(())
    }

    pub fn erase_pixels(
        &mut self,
        config: &ConfigModel,
        pixels: impl Iterator<Item = PixelPosition>,
    ) -> Result<()> {
        let mut command = PixelCanvasCommand::default();
        for position in pixels {
            if let Some(color) = self.pixels.get_pixel(position) {
                command.erase.push(Pixel::new(position, color));
            }
        }
        command.erase.sort_by_key(|x| x.position);
        self.apply_command(config, command).or_fail()?;
        Ok(())
    }

    pub fn erase_and_draw_pixels(
        &mut self,
        config: &ConfigModel,
        erase_pixels: impl Iterator<Item = PixelPosition>,
        draw_pixels: impl Iterator<Item = Pixel>,
    ) -> Result<()> {
        let mut command = PixelCanvasCommand::default();
        for position in erase_pixels {
            if let Some(color) = self.pixels.get_pixel(position) {
                command.erase.push(Pixel::new(position, color));
            }
        }
        command.erase.sort_by_key(|x| x.position);

        let mut overwritten = Vec::new();
        for pixel in draw_pixels {
            let color = if command
                .erase
                .binary_search_by_key(&pixel.position, |x| x.position)
                .is_ok()
            {
                pixel.color
            } else if let Some(old_color) = self.pixels.get_pixel(pixel.position) {
                overwritten.push(Pixel::new(pixel.position, old_color));
                pixel.color.alpha_blend(old_color)
            } else {
                pixel.color
            };
            command.draw.push(Pixel::new(pixel.position, color));
        }
        command.erase.extend(overwritten);
        command.erase.sort_by_key(|x| x.position);
        command.erase.dedup();
        command.draw.sort_by_key(|x| x.position);

        self.apply_command(config, command).or_fail()?;
        Ok(())
    }

    pub fn replace_color(&mut self, config: &ConfigModel, old: Rgba, new: Rgba) -> Result<()> {
        // TODO: optimize (e.g., to use cache to get target pixels)

        let mut command = PixelCanvasCommand::default();
        for (&position, &color) in &self.pixels.0 {
            if color != old {
                continue;
            }

            command.erase.push(Pixel::new(position, old));
            command.draw.push(Pixel::new(position, new));
        }
        command.draw.sort_by_key(|x| x.position);
        command.erase.sort_by_key(|x| x.position);

        self.apply_command(config, command).or_fail()?;
        Ok(())
    }

    fn apply_command(&mut self, config: &ConfigModel, command: PixelCanvasCommand) -> Result<()> {
        if command.erase.is_empty() && command.draw.is_empty() {
            return Ok(());
        }

        self.command_log.truncate(self.command_log_tail);
        self.command_log.push_back(command);
        self.redo_command(config).or_fail()?;

        Ok(())
    }

    pub fn region(&self) -> PixelRegion {
        PixelRegion::from_positions(self.pixels.0.keys().copied())
    }

    pub fn get_pixels(
        &self,
        config: &ConfigModel,
        region: PixelRegion,
    ) -> impl '_ + Iterator<Item = Pixel> {
        // TODO: optimize (e.g., use cache to avoid redundant calculation)
        let frame = config.frame;
        let layer = config.layer;
        let frame_count = config.animation.enabled_frame_count();
        region.pixels().filter_map(move |position| {
            self.get_layered_pixel(frame, frame_count, layer, position)
                .map(|color| Pixel::new(position, color))
        })
    }

    pub fn get_pixel(&self, config: &ConfigModel, position: PixelPosition) -> Option<Rgba> {
        self.get_layered_pixel(
            config.frame,
            config.animation.enabled_frame_count(),
            config.layer,
            position,
        )
    }

    pub fn get_pixel_with_alpha(
        &self,
        config: &ConfigModel,
        position: PixelPosition,
        alpha: u8,
    ) -> Option<Rgba> {
        let layer = config.layer;
        let frame = config.frame;
        let frame_count = config.animation.enabled_frame_count();
        let mut color = None;
        layer.for_each_lower_layer_pixel_but_last(frame, frame_count, position, |position| {
            if let Some(c) = self.pixels.get_pixel(position) {
                color = Some(color.map_or(c, |d| c.alpha_blend(d)));
            }
        });

        if alpha == 0 {
            return color;
        }

        if let Some(mut c) = self.get_direct_pixel(position) {
            c.a = alpha;
            Some(color.map_or(c, |d| c.alpha_blend(d)))
        } else {
            color
        }
    }

    pub fn get_direct_pixel(&self, position: PixelPosition) -> Option<Rgba> {
        self.pixels.get_pixel(position)
    }

    fn get_layered_pixel(
        &self,
        frame: FrameRegion,
        frame_count: u16,
        layer: Layer,
        position: PixelPosition,
    ) -> Option<Rgba> {
        let mut color = None;
        layer.for_each_lower_layer_pixel(frame, frame_count, position, |position| {
            if let Some(c) = self.pixels.get_pixel(position) {
                color = Some(color.map_or(c, |d| c.alpha_blend(d)));
            }
        });
        color
    }

    pub fn undo_command(&mut self, config: &ConfigModel) -> Result<()> {
        if let Some(i) = self.command_log_tail.checked_sub(1) {
            let layer = config.layer;
            let frame = config.frame;
            let frame_count = config.animation.enabled_frame_count();
            let command = &self.command_log[i];
            for &pixel in &command.draw {
                self.pixels.erase_pixel(pixel).or_fail()?;
                layer.for_each_upper_layer_pixel(frame, frame_count, pixel.position, |position| {
                    self.dirty_positions.insert(position);
                });
            }
            for &pixel in &command.erase {
                self.pixels.draw_pixel(pixel).or_fail()?;
                layer.for_each_upper_layer_pixel(frame, frame_count, pixel.position, |position| {
                    self.dirty_positions.insert(position);
                });
            }
            self.command_log_tail = i;
        }
        self.state_version -= 1;
        Ok(())
    }

    pub fn redo_command(&mut self, config: &ConfigModel) -> Result<()> {
        if let Some(command) = self.command_log.get(self.command_log_tail) {
            let layer = config.layer;
            let frame = config.frame;
            let frame_count = config.animation.enabled_frame_count();
            for &pixel in &command.erase {
                self.pixels.erase_pixel(pixel).or_fail()?;
                layer.for_each_upper_layer_pixel(frame, frame_count, pixel.position, |position| {
                    self.dirty_positions.insert(position);
                });
            }
            for &pixel in &command.draw {
                self.pixels.draw_pixel(pixel).or_fail()?;
                layer.for_each_upper_layer_pixel(frame, frame_count, pixel.position, |position| {
                    self.dirty_positions.insert(position);
                });
            }
            self.command_log_tail += 1;
        }
        self.state_version += 1;
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
            state_version: 0,
        })
    }
}

#[derive(Debug, Default)]
pub struct PixelCanvasCommand {
    pub erase: Vec<Pixel>,
    pub draw: Vec<Pixel>,
}

impl PixelCanvasCommand {
    fn new() -> Self {
        Self::default()
    }
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
    fn get_pixel(&self, position: PixelPosition) -> Option<Rgba> {
        self.0.get(&position).copied()
    }

    fn draw_pixel(&mut self, pixel: Pixel) -> Result<()> {
        let prev = if pixel.color.a == 0 {
            self.0.remove(&pixel.position)
        } else {
            self.0.insert(pixel.position, pixel.color)
        };
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
        y += delta;
        ys.push(y);
    }

    let mut positions = Vec::with_capacity(size);
    let mut x = 0;
    for y in ys {
        let delta = i16::deserialize(reader).or_fail()?;
        x += delta;
        positions.push(PixelPosition::from_xy(x, y));
    }

    Ok(positions.into_iter())
}
