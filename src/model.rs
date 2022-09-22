use self::{config::ConfigModel, pixel_canvas::PixelCanvasModel, tool::ToolModel};
use crate::serialize::{Deserialize, Serialize};
use pagurus::{
    failure::{Failure, OrFail},
    Result,
};
use pagurus_game_std::color::Rgba;
use png::chunk::ChunkType;
use std::io::{Read, Write};

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
}

impl Models {
    pub fn to_png(&self) -> Result<Vec<u8>> {
        let image_data = self
            .config
            .frame
            .get()
            .pixels()
            .flat_map(|position| {
                let color = self
                    .pixel_canvas
                    .get_pixel(position)
                    .unwrap_or(Rgba::new(0, 0, 0, 0));
                [color.r, color.g, color.b, color.a].into_iter()
            })
            .collect::<Vec<_>>();
        let image_size = self.config.frame.get().size();

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
            let mut writer = encoder.write_header().or_fail()?;
            writer.write_image_data(&image_data).or_fail()?;
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
        Err(Failure::new(format!(
            "No {PNG_CHUNK_TYPE:?} chunk in PNG file"
        )))
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
