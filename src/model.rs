use self::{config::ConfigModel, pixel_canvas::PixelCanvasModel, tool::ToolModel};
use crate::pixel::PixelSize;
use crate::png::decode_sprite;
use crate::{
    pixel::{Pixel, PixelPosition},
    serialize::{Deserialize, Serialize},
};
use orfail::{OrFail, Result};
use pagurus::image::Rgba;
use png::chunk::ChunkType;
use std::collections::HashSet;
use std::io::{Read, Write};

pub mod attributes;
pub mod config;
pub mod pixel_canvas;
pub mod tool;

pub const PNG_CHUNK_TYPE: ChunkType = ChunkType(*b"siLE");
pub const MAGIC_NUMBER: [u8; 6] = *b"PIXCIL";
pub const FORMAT_VERSION: u16 = 0;

#[derive(Debug, Default)]
pub struct Models {
    pub config: ConfigModel,
    pub pixel_canvas: PixelCanvasModel,

    // The following fields are not serialized / deserialized.
    pub tool: ToolModel,
    pub preview_mode: bool,
}

impl Models {
    pub fn frame_size(&self) -> PixelSize {
        self.config.frame.get_base_region().size()
    }

    pub fn palette(&self) -> HashSet<Rgba> {
        let mut palette = HashSet::new();
        let frame_count = self.config.animation.enabled_frame_count();
        for frame in 0..frame_count {
            for position in self
                .config
                .frame
                .get_preview_region(&self.config, frame as usize)
                .pixels()
            {
                let color = self
                    .pixel_canvas
                    .get_pixel(&self.config, position)
                    .unwrap_or(Rgba::new(0, 0, 0, 0));
                if color.a > 0 {
                    palette.insert(color);
                }
            }
        }
        palette
    }

    pub fn to_png(&self) -> Result<Vec<u8>> {
        let bg_color = self
            .config
            .background_color
            .unwrap_or(Rgba::new(0, 0, 0, 0));
        let frame_count = self.config.animation.enabled_frame_count();
        let frames = (0..frame_count)
            .map(|frame| {
                self.config
                    .frame
                    .get_preview_region(&self.config, frame as usize)
                    .pixels()
                    .flat_map(|position| {
                        let color =
                            if let Some(c) = self.pixel_canvas.get_pixel(&self.config, position) {
                                c.alpha_blend(bg_color)
                            } else {
                                bg_color
                            };
                        [color.r, color.g, color.b, color.a].into_iter()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let image_size = self.config.frame.get_base_region().size();

        let mut metadata = Vec::new();
        self.serialize(&mut metadata).or_fail()?;

        let mut png_data = Vec::new();
        {
            let mut encoder = png::Encoder::new(
                &mut png_data,
                u32::from(image_size.width),
                u32::from(image_size.height),
            );
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_compression(png::Compression::Best);

            if frame_count > 1 {
                encoder.set_animated(frame_count as u32, 0).or_fail()?;
                encoder
                    .set_frame_delay(1, self.config.animation.fps() as u16)
                    .or_fail()?;
            }

            let mut writer = encoder.write_header().or_fail()?;
            for image_data in &frames {
                writer.write_image_data(image_data).or_fail()?;
            }
            writer.write_chunk(PNG_CHUNK_TYPE, &metadata).or_fail()?;
        }
        Ok(png_data)
    }

    pub fn from_png(png_data: &[u8]) -> Result<Self> {
        let mut decoder = png::StreamingDecoder::new();
        decoder.set_ignore_text_chunk(true);
        let mut offset = 0;
        while offset < png_data.len() {
            let mut buf = Vec::new();
            let (read_size, decoded) = decoder.update(&png_data[offset..], &mut buf).or_fail()?;
            offset += read_size;

            if let png::Decoded::ChunkBegin(_, PNG_CHUNK_TYPE) = decoded {
                let mut reader = &png_data[offset..];
                let models = Self::deserialize(&mut reader).or_fail()?;
                return Ok(models);
            }
        }

        // Load the image with the default settings.
        //
        // TODO: Support animated PNG
        let mut models = Self::default();
        let image = decode_sprite(png_data).or_fail()?;
        models
            .pixel_canvas
            .draw_pixels(
                &models.config,
                image.pixels().map(|(pos, rgba)| {
                    Pixel::new(PixelPosition::from_xy(pos.x as i16, pos.y as i16), rgba)
                }),
            )
            .or_fail()?;
        models.pixel_canvas.forget_oldest_command();
        models.config.frame.set_width(image.size().width as u16);
        models.config.frame.set_height(image.size().height as u16);

        Ok(models)
    }
}

impl Serialize for Models {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        MAGIC_NUMBER.serialize(writer).or_fail()?;
        FORMAT_VERSION.serialize(writer).or_fail()?;

        let config_size = u16::try_from(self.config.serialized_size().or_fail()?).or_fail()?;
        config_size.serialize(writer).or_fail()?;
        self.config.serialize(writer).or_fail()?;

        self.pixel_canvas.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Models {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let magic_number: [u8; 6] = Deserialize::deserialize(reader).or_fail()?;
        (magic_number == MAGIC_NUMBER).or_fail()?;

        let version: u16 = Deserialize::deserialize(reader).or_fail()?;
        (version == FORMAT_VERSION).or_fail()?;

        let config_size = u16::deserialize(reader).or_fail()?;
        let config;
        {
            let mut reader = reader.take(u64::from(config_size));
            config = ConfigModel::deserialize(&mut reader).or_fail()?;
            // Ignore unknown fields.
            for _ in reader.bytes() {}
        };

        Ok(Self {
            config,
            pixel_canvas: Deserialize::deserialize(reader).or_fail()?,
            ..Default::default()
        })
    }
}
