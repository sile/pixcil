use orfail::{OrFail, Result};
use pagurus::{image::Sprite, spatial::Size};

pub fn decode_sprite(png: &[u8]) -> Result<Sprite> {
    let decoder = png::Decoder::new(png);
    let mut reader = decoder.read_info().or_fail()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).or_fail()?;
    let bytes = &buf[..info.buffer_size()];
    let size = Size::from_wh(info.width, info.height);
    (info.bit_depth == png::BitDepth::Eight)
        .or_fail_with(|_| format!("unsupported PNG bit depth: {:?}", info.bit_depth))?;

    match info.color_type {
        png::ColorType::Rgb => Sprite::from_rgb24_bytes(bytes, size).or_fail(),
        png::ColorType::Rgba => Sprite::from_rgba32_bytes(bytes, size).or_fail(),
        png::ColorType::Grayscale => Sprite::from_grayscale8_bytes(bytes, size).or_fail(),
        png::ColorType::GrayscaleAlpha => {
            Sprite::from_grayscale_alpha16_bytes(bytes, size).or_fail()
        }
        png::ColorType::Indexed => {
            let palette = reader.info().palette.as_ref().or_fail()?;
            let mut rgb_bytes = Vec::with_capacity(size.len());
            for i in bytes.iter().copied().map(usize::from) {
                rgb_bytes.push(palette[i * 3]);
                rgb_bytes.push(palette[i * 3 + 1]);
                rgb_bytes.push(palette[i * 3 + 2]);
            }
            Sprite::from_rgb24_bytes(&rgb_bytes, size).or_fail()
        }
    }
}

pub fn decode_sprites(png: &[u8]) -> Result<Vec<Sprite>> {
    let decoder = png::Decoder::new(png);
    let mut reader = decoder.read_info().or_fail()?;
    let mut buf = vec![0; reader.output_buffer_size()];

    let mut sprites = Vec::new();
    loop {
        // TODO: Consider reader.info()
        let info = match reader.next_frame(&mut buf).or_fail() {
            Ok(info) => info,
            Err(e) if sprites.is_empty() => {
                return Err(e);
            }
            Err(_) => break,
        };
        let bytes = &buf[..info.buffer_size()];
        let size = Size::from_wh(info.width, info.height);
        (info.bit_depth == png::BitDepth::Eight)
            .or_fail_with(|_| format!("unsupported PNG bit depth: {:?}", info.bit_depth))?;

        let sprite = match info.color_type {
            png::ColorType::Rgb => Sprite::from_rgb24_bytes(bytes, size).or_fail(),
            png::ColorType::Rgba => Sprite::from_rgba32_bytes(bytes, size).or_fail(),
            png::ColorType::Grayscale => Sprite::from_grayscale8_bytes(bytes, size).or_fail(),
            png::ColorType::GrayscaleAlpha => {
                Sprite::from_grayscale_alpha16_bytes(bytes, size).or_fail()
            }
            png::ColorType::Indexed => {
                let palette = reader.info().palette.as_ref().or_fail()?;
                let mut rgb_bytes = Vec::with_capacity(size.len());
                for i in bytes.iter().copied().map(usize::from) {
                    rgb_bytes.push(palette[i * 3]);
                    rgb_bytes.push(palette[i * 3 + 1]);
                    rgb_bytes.push(palette[i * 3 + 2]);
                }
                Sprite::from_rgb24_bytes(&rgb_bytes, size).or_fail()
            }
        }?;
        sprites.push(sprite);
    }
    Ok(sprites)
}
