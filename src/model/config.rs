use super::attributes::AttributesModel;
use crate::{
    app::App,
    pixel::{PixelPosition, PixelRegion, PixelSize},
    serialize::{Deserialize, Serialize},
};
use orfail::{OrFail, Result};
use pagurus::image::Rgba;
use pagurus::spatial::Position;
use std::{
    io::{Read, Write},
    time::Duration,
};

#[derive(Debug, Default, Clone)]
pub struct ConfigModel {
    pub zoom: Zoom,
    pub camera: Camera,
    pub minimum_pixel_size: MinimumPixelSize,
    // TODO: remove this field in the future version
    pub max_undos: MaxUndos,
    pub color: DrawingColor,
    pub frame: FrameRegion,
    pub frame_preview: FramePreview,
    pub layer: Layer,
    pub animation: Animation,
    pub finger_mode: FingerMode,
    pub frame_preview_scale: FramePreviewScale,

    // TODO: remove this field in the future version
    pub attrs: AttributesModel,

    pub silhouette_preview: bool,
    pub gesture: bool,
    pub background_color: Option<Rgba>,
    pub apng: Option<bool>,
}

impl ConfigModel {
    pub fn apng(&self) -> bool {
        self.apng.unwrap_or(true)
    }
}

impl Serialize for ConfigModel {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.zoom.serialize(writer).or_fail()?;
        self.camera.serialize(writer).or_fail()?;
        self.minimum_pixel_size.serialize(writer).or_fail()?;
        self.max_undos.serialize(writer).or_fail()?;
        self.color.serialize(writer).or_fail()?;
        self.frame.serialize(writer).or_fail()?;
        self.frame_preview.serialize(writer).or_fail()?;
        self.layer.serialize(writer).or_fail()?;
        self.animation.serialize(writer).or_fail()?;
        self.finger_mode.serialize(writer).or_fail()?;
        self.frame_preview_scale.serialize(writer).or_fail()?;
        self.attrs.serialize(writer).or_fail()?;
        self.silhouette_preview.serialize(writer).or_fail()?;
        self.gesture.serialize(writer).or_fail()?;
        self.background_color.serialize(writer).or_fail()?;
        self.apng.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for ConfigModel {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            zoom: Deserialize::deserialize_or_default(reader).or_fail()?,
            camera: Deserialize::deserialize_or_default(reader).or_fail()?,
            minimum_pixel_size: Deserialize::deserialize_or_default(reader).or_fail()?,
            max_undos: Deserialize::deserialize_or_default(reader).or_fail()?,
            color: Deserialize::deserialize_or_default(reader).or_fail()?,
            frame: Deserialize::deserialize_or_default(reader).or_fail()?,
            frame_preview: Deserialize::deserialize_or_default(reader).or_fail()?,
            layer: Deserialize::deserialize_or_default(reader).or_fail()?,
            animation: Deserialize::deserialize_or_default(reader).or_fail()?,
            finger_mode: Deserialize::deserialize_or_default(reader).or_fail()?,
            frame_preview_scale: Deserialize::deserialize_or_default(reader).or_fail()?,
            attrs: Deserialize::deserialize_or_default(reader).or_fail()?,
            silhouette_preview: Deserialize::deserialize_or_default(reader).or_fail()?,
            gesture: Deserialize::deserialize_or_default(reader).or_fail()?,
            background_color: Deserialize::deserialize_or_default(reader).or_fail()?,
            apng: Deserialize::deserialize_or_default(reader).or_fail()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Zoom(u8);

impl Zoom {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(50);

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn is_min(self) -> bool {
        self.0 == Self::MIN.0
    }

    pub const fn is_max(self) -> bool {
        self.0 == Self::MAX.0
    }

    pub fn decrement(&mut self) {
        self.0 = std::cmp::max(Self::MIN.0, self.0 - 1);
    }

    pub fn increment(&mut self) {
        self.0 = std::cmp::min(Self::MAX.0, self.0 + 1);
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom(8)
    }
}

impl Serialize for Zoom {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for Zoom {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let n = u8::deserialize(reader).map(Self).or_fail()?;
        Ok(n.clamp(Self::MIN, Self::MAX))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Camera(Position);

impl Camera {
    pub const MIN: Self = Self(Position::from_xy(i32::MIN / 2, i32::MIN / 2));
    pub const MAX: Self = Self(Position::from_xy(i32::MAX / 2, i32::MAX / 2));

    pub const fn get(self) -> Position {
        self.0
    }

    pub fn r#move(&mut self, delta: Position) {
        self.0.x = (self.0.x + delta.x).clamp(Self::MIN.0.x, Self::MAX.0.x);
        self.0.y = (self.0.y + delta.y).clamp(Self::MIN.0.y, Self::MAX.0.y);
    }

    pub fn frame_center(self, app: &App, frame: usize, layer: usize) -> Position {
        let region = app
            .models()
            .config
            .frame
            .get_base_region()
            .shift_x(frame as i16)
            .shift_y(layer as i16);
        region.center().to_screen_position(app)
    }

    pub fn current_frame_center(self, app: &App) -> Position {
        let layer = self.current_layer(app);
        let frame = self.current_frame(app);
        self.frame_center(app, frame, layer)
    }

    pub fn current_layer(self, app: &App) -> usize {
        let config = &app.models().config;
        let layer_count = config.layer.enabled_count();
        if layer_count == 1 {
            0
        } else {
            let frame_height = config.frame.get_base_region().size().height;
            let base_frame_position = config.frame.get_base_region().start;
            let screen_center = app.screen_size().to_region().center();
            let position = PixelPosition::from_screen_position(app, screen_center);
            let index = (position.y - base_frame_position.y) / frame_height as i16;
            index.clamp(0, layer_count as i16 - 1) as usize
        }
    }

    pub fn current_frame(self, app: &App) -> usize {
        let config = &app.models().config;
        let frame_count = config.animation.enabled_frame_count();
        if frame_count == 1 {
            0
        } else {
            let frame_width = config.frame.get_base_region().size().width;
            let base_frame_position = config.frame.get_base_region().start;
            let screen_center = app.screen_size().to_region().center();
            let position = PixelPosition::from_screen_position(app, screen_center);
            let index = (position.x - base_frame_position.x) / frame_width as i16;
            index.clamp(0, frame_count as i16 - 1) as usize
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let zoom = i32::from(Zoom::default().get());
        let frame = FrameRegion::default().get_base_region().size();

        Self(Position::from_xy(
            i32::from(frame.width) * zoom / 2,
            i32::from(frame.height) * zoom / 2,
        ))
    }
}

impl Serialize for Camera {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for Camera {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let p = Position::deserialize(reader).or_fail()?;
        let x = p.x.clamp(Self::MIN.0.x, Self::MAX.0.x);
        let y = p.y.clamp(Self::MIN.0.y, Self::MAX.0.y);
        Ok(Self(Position::from_xy(x, y)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinimumPixelSize(PixelSize);

impl MinimumPixelSize {
    pub const MIN: Self = Self(PixelSize::square(1));
    pub const MAX: Self = Self(PixelSize::square(999));

    pub const fn get(self) -> PixelSize {
        self.0
    }

    pub fn set(&mut self, size: PixelSize) {
        self.0.width = size.width.clamp(Self::MIN.0.width, Self::MAX.0.width);
        self.0.height = size.height.clamp(Self::MIN.0.height, Self::MAX.0.height);
    }

    pub fn set_delta(&mut self, delta: i16) {
        let mut s = self.0;
        s.width = s.width.saturating_add_signed(delta);
        s.height = s.height.saturating_add_signed(delta);
        self.set(s);
    }

    pub fn normalize(self, mut position: PixelPosition) -> PixelPosition {
        let w = self.0.width as i16;
        if position.x >= 0 {
            position.x /= w;
        } else {
            position.x = (position.x - (w - 1)) / w;
        }

        let h = self.0.height as i16;
        if position.y >= 0 {
            position.y /= h;
        } else {
            position.y = (position.y - (h - 1)) / h;
        }
        position
    }

    pub fn denormalize(self, mut position: PixelPosition) -> PixelPosition {
        position.x *= self.0.width as i16;
        position.y *= self.0.height as i16;
        position
    }

    pub fn align(self, position: PixelPosition) -> PixelPosition {
        self.denormalize(self.normalize(position))
    }

    pub fn denormalize_to_region(self, position: PixelPosition) -> PixelRegion {
        let start = self.denormalize(position);
        let mut end = start;
        end.x += self.0.width as i16;
        end.y += self.0.height as i16;
        PixelRegion { start, end }
    }

    pub fn to_region(self, position: PixelPosition) -> PixelRegion {
        self.denormalize_to_region(self.normalize(position))
    }
}

impl Default for MinimumPixelSize {
    fn default() -> Self {
        Self(PixelSize::square(1))
    }
}

impl Serialize for MinimumPixelSize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for MinimumPixelSize {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut size = PixelSize::deserialize(reader).or_fail()?;
        size.width = size.width.clamp(Self::MIN.0.width, Self::MAX.0.width);
        size.height = size.height.clamp(Self::MIN.0.height, Self::MAX.0.height);
        Ok(Self(size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawingColor(Rgba);

impl DrawingColor {
    pub const fn get(self) -> Rgba {
        self.0
    }

    pub fn set(&mut self, color: Rgba) {
        self.0 = color;
    }
}

impl Default for DrawingColor {
    fn default() -> Self {
        Self(Rgba::new(0, 0, 0, 255))
    }
}

impl Serialize for DrawingColor {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for DrawingColor {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Rgba::deserialize(reader).map(Self).or_fail()
    }
}

// TODO: Rename s/FrameRegion/Frame/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameRegion {
    preview: bool,
    region: PixelRegion,
}

impl FrameRegion {
    pub const fn get_base_region(self) -> PixelRegion {
        self.region
    }

    // TODO: rename
    pub fn get_animation_frames_region(self, config: &ConfigModel) -> PixelRegion {
        let mut region = self.region.shift_y(config.layer.enabled_count() as i16 - 1);
        let size = self.region.size();
        region.end.x = size.width as i16 * config.animation.enabled_frame_count() as i16;
        region
    }

    // TODO: rename
    pub fn get_full_region(self, config: &ConfigModel) -> PixelRegion {
        let mut region = self.region;
        let size = self.region.size();
        region.end.x = size.width as i16 * config.animation.enabled_frame_count() as i16;
        region.end.y = size.height as i16 * config.layer.enabled_count() as i16;
        region
    }

    pub fn get_preview_region(self, config: &ConfigModel, frame: usize) -> PixelRegion {
        let layers = config.layer.enabled_count() - 1;
        let region = self.get_base_region();
        region
            .move_y(region.size().height as i16 * layers as i16)
            .move_x(region.size().width as i16 * frame as i16)
    }

    pub fn set_from_pixel_size(&mut self, size: PixelSize) {
        self.region = PixelRegion::from_position_and_size(self.region.start, size);
    }

    pub fn set_width(&mut self, width: u16) {
        let mut size = self.region.size();
        size.width = width;
        self.region = PixelRegion::from_position_and_size(self.region.start, size);
    }

    pub fn set_height(&mut self, height: u16) {
        let mut size = self.region.size();
        size.height = height;
        self.region = PixelRegion::from_position_and_size(self.region.start, size);
    }
}

impl Default for FrameRegion {
    fn default() -> Self {
        Self {
            preview: true,
            region: PixelRegion::new(PixelPosition::from_xy(0, 0), PixelPosition::from_xy(64, 64)),
        }
    }
}

impl Serialize for FrameRegion {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.preview.serialize(writer).or_fail()?;
        self.region.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for FrameRegion {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            preview: bool::deserialize(reader).or_fail()?,
            region: PixelRegion::deserialize(reader).or_fail()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FramePreviewScale(u8);

impl FramePreviewScale {
    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, scale: u8) -> Result<()> {
        (scale > 0).or_fail()?;
        self.0 = scale;
        Ok(())
    }
}

impl Default for FramePreviewScale {
    fn default() -> Self {
        Self(1)
    }
}

impl Serialize for FramePreviewScale {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for FramePreviewScale {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(u8::deserialize(reader).or_fail()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FramePreview(bool);

impl FramePreview {
    pub const fn get(self) -> bool {
        self.0
    }

    pub fn set(&mut self, on: bool) {
        self.0 = on;
    }
}

impl Default for FramePreview {
    fn default() -> Self {
        Self(true)
    }
}

impl Serialize for FramePreview {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for FramePreview {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(bool::deserialize(reader).or_fail()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaxUndos(u32);

impl Default for MaxUndos {
    fn default() -> Self {
        Self(100)
    }
}

impl Serialize for MaxUndos {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.0.serialize(writer).or_fail()
    }
}

impl Deserialize for MaxUndos {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        u32::deserialize(reader).map(Self).or_fail()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Layer {
    enabled: bool,
    count: u16,
}

impl Layer {
    pub const MIN: u16 = 1;
    pub const MAX: u16 = 10;

    pub const fn is_enabled(self) -> bool {
        self.enabled
    }

    pub const fn count(self) -> u16 {
        self.count
    }

    pub fn enabled_count(self) -> u16 {
        if self.enabled { self.count } else { 1 }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_count(&mut self, n: u16) {
        self.count = n.clamp(Self::MIN, Self::MAX);
    }

    pub fn for_each_lower_layer_pixel<F>(
        self,
        frame: FrameRegion,
        frames: u16,
        position: PixelPosition,
        mut f: F,
    ) where
        F: FnMut(PixelPosition),
    {
        let layers = self.enabled_count();
        if layers == 1 {
            f(position);
            return;
        }

        let mut frame = frame.get_base_region();
        frame.set_width(frame.size().width * frames);
        if frame.contains(position) {
            f(position);
            return;
        }

        let layer_region = PixelRegion::from_position_and_size(
            frame.start,
            PixelSize::from_wh(frame.size().width, frame.size().height * layers),
        );
        if !layer_region.contains(position) {
            f(position);
            return;
        }

        let mut current = PixelPosition::from_xy(
            position.x,
            (position.y - frame.start.y) % frame.size().height as i16 + frame.start.y,
        );
        for _ in 0..=layers {
            f(current);
            if current == position {
                break;
            }
            current.y += frame.size().height as i16;
        }
    }

    // TODO: refactor
    pub fn for_each_lower_layer_pixel_but_last<F>(
        self,
        frame: FrameRegion,
        frames: u16,
        position: PixelPosition,
        mut f: F,
    ) where
        F: FnMut(PixelPosition),
    {
        let layers = self.enabled_count();
        if layers == 1 {
            return;
        }

        let mut frame = frame.get_base_region();
        frame.set_width(frame.size().width * frames);
        if frame.contains(position) {
            return;
        }

        let layer_region = PixelRegion::from_position_and_size(
            frame.start,
            PixelSize::from_wh(frame.size().width, frame.size().height * layers),
        );
        if !layer_region.contains(position) {
            return;
        }

        let mut current = PixelPosition::from_xy(
            position.x,
            (position.y - frame.start.y) % frame.size().height as i16 + frame.start.y,
        );
        for _ in 0..=layers {
            f(current);
            current.y += frame.size().height as i16;
            if current == position {
                break;
            }
        }
    }

    pub fn for_each_upper_layer_pixel<F>(
        self,
        frame: FrameRegion,
        frames: u16,
        position: PixelPosition,
        mut f: F,
    ) where
        F: FnMut(PixelPosition),
    {
        let layers = self.enabled_count();
        if layers == 1 {
            f(position);
            return;
        }

        let frame = frame.get_base_region();
        let layer_region = PixelRegion::from_position_and_size(
            frame.start,
            PixelSize::from_wh(frame.size().width * frames, frame.size().height * layers),
        );
        if !layer_region.contains(position) {
            f(position);
            return;
        }

        let mut current = position;
        while layer_region.contains(current) {
            f(current);
            current.y += frame.size().height as i16;
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            enabled: false,
            count: 1,
        }
    }
}

impl Serialize for Layer {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.enabled.serialize(writer).or_fail()?;
        self.count.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Layer {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            enabled: Deserialize::deserialize(reader).or_fail()?,
            count: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Animation {
    enabled: bool,
    fps: u8,
    frame_count: u16,
}

impl Animation {
    pub const MIN_FPS: u8 = 1;
    pub const MAX_FPS: u8 = 120;

    pub const MIN_FRAME_COUNT: u16 = 1;
    pub const MAX_FRAME_COUNT: u16 = 1000;

    pub const fn is_enabled(self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub const fn fps(self) -> u8 {
        self.fps
    }

    pub fn set_fps(&mut self, fps: u8) {
        self.fps = fps.clamp(Self::MIN_FPS, Self::MAX_FPS);
    }

    pub const fn frame_count(self) -> u16 {
        self.frame_count
    }

    pub fn enabled_frame_count(self) -> u16 {
        if self.enabled { self.frame_count } else { 1 }
    }

    pub fn set_frame_count(&mut self, n: u16) {
        self.frame_count = n.clamp(Self::MIN_FRAME_COUNT, Self::MAX_FRAME_COUNT);
    }

    pub fn frame_interval(self) -> Duration {
        Duration::from_secs(1) / u32::from(self.fps)
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            enabled: false,
            fps: 10,
            frame_count: 1,
        }
    }
}

impl Serialize for Animation {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.enabled.serialize(writer).or_fail()?;
        self.fps.serialize(writer).or_fail()?;
        self.frame_count.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for Animation {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            enabled: Deserialize::deserialize(reader).or_fail()?,
            fps: Deserialize::deserialize(reader).or_fail()?,
            frame_count: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}

// TODO: Remove this struct in the future version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FingerMode {
    enabled: bool,
    cursor_distance: u32,
}

impl FingerMode {
    pub const fn enabled(self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub const fn cursor_distance(self) -> u32 {
        self.cursor_distance
    }

    pub fn set_cursor_distance(&mut self, distance: u32) {
        self.cursor_distance = distance;
    }
}

impl Default for FingerMode {
    fn default() -> Self {
        Self {
            enabled: false,
            cursor_distance: 150,
        }
    }
}

impl Serialize for FingerMode {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.enabled.serialize(writer).or_fail()?;
        self.cursor_distance.serialize(writer).or_fail()?;
        Ok(())
    }
}

impl Deserialize for FingerMode {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            enabled: Deserialize::deserialize(reader).or_fail()?,
            cursor_distance: Deserialize::deserialize(reader).or_fail()?,
        })
    }
}
