use pagurus::spatial::{Region, Size};
use pagurus::{failure::OrFail, Result};
use pagurus_game_std::image::Sprite;
use pagurus_game_std::png::decode_sprite;

#[derive(Debug)]
pub struct Assets {
    pub icons: Icons,
    pub buttons: Buttons,
    pub digits_10x14: [Sprite; 10],
}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Self {
            icons: Icons::load().or_fail()?,
            buttons: Buttons::load().or_fail()?,
            digits_10x14: load_digits_10x14().or_fail()?,
        })
    }

    pub fn get_icon(&self, id: IconId) -> &Sprite {
        match id {
            IconId::Undo => &self.icons.undo,
            IconId::Redo => &self.icons.redo,
            IconId::ZoomIn => &self.icons.zoom_in,
            IconId::ZoomOut => &self.icons.zoom_out,
            IconId::Null => &self.icons.null,
            IconId::Settings => &self.icons.settings,
            IconId::Draw => &self.icons.draw,
            IconId::Erase => &self.icons.erase,
            IconId::Select => &self.icons.select,
            IconId::Pick => &self.icons.pick,
            IconId::Move => &self.icons.r#move,
            IconId::Save => &self.icons.save,
            IconId::Load => &self.icons.load,
            IconId::Import => &self.icons.import,
        }
    }

    pub fn get_button(&self, kind: ButtonKind) -> &Button {
        match kind {
            ButtonKind::Basic => &self.buttons.basic,
            ButtonKind::BasicDeep => &self.buttons.basic_deep,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconId {
    Undo,
    Redo,
    ZoomIn,
    ZoomOut,
    Null,
    Settings,
    Draw,
    Erase,
    Select,
    Pick,
    Move,
    Save,
    Load,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Basic,
    BasicDeep,
}

impl ButtonKind {
    pub fn size(self) -> Size {
        match self {
            ButtonKind::Basic => Size::square(64),
            ButtonKind::BasicDeep => Size::square(64),
        }
    }
}

#[derive(Debug)]
pub struct Icons {
    pub undo: Sprite,
    pub redo: Sprite,
    pub zoom_in: Sprite,
    pub zoom_out: Sprite,
    pub null: Sprite,
    pub settings: Sprite,
    pub draw: Sprite,
    pub erase: Sprite,
    pub select: Sprite,
    pub pick: Sprite,
    pub r#move: Sprite,
    pub save: Sprite,
    pub load: Sprite,
    pub import: Sprite,
}

impl Icons {
    fn load() -> Result<Self> {
        Ok(Self {
            undo: decode_sprite(include_bytes!("../assets/icon-undo.png")).or_fail()?,
            redo: decode_sprite(include_bytes!("../assets/icon-redo.png")).or_fail()?,
            zoom_in: decode_sprite(include_bytes!("../assets/icon-zoom-in.png")).or_fail()?,
            zoom_out: decode_sprite(include_bytes!("../assets/icon-zoom-out.png")).or_fail()?,
            null: decode_sprite(include_bytes!("../assets/icon-null.png")).or_fail()?,
            settings: decode_sprite(include_bytes!("../assets/icon-settings.png")).or_fail()?,
            draw: decode_sprite(include_bytes!("../assets/icon-draw.png")).or_fail()?,
            erase: decode_sprite(include_bytes!("../assets/icon-erase.png")).or_fail()?,
            select: decode_sprite(include_bytes!("../assets/icon-select.png")).or_fail()?,
            pick: decode_sprite(include_bytes!("../assets/icon-color-pick.png")).or_fail()?,
            r#move: decode_sprite(include_bytes!("../assets/icon-move.png")).or_fail()?,
            save: decode_sprite(include_bytes!("../assets/icon-save.png")).or_fail()?,
            load: decode_sprite(include_bytes!("../assets/icon-load.png")).or_fail()?,
            import: decode_sprite(include_bytes!("../assets/icon-import.png")).or_fail()?,
        })
    }
}

#[derive(Debug)]
pub struct Buttons {
    pub basic: Button,
    pub basic_deep: Button,
}

impl Buttons {
    fn load() -> Result<Self> {
        let basic = Button::new(
            decode_sprite(include_bytes!("../assets/button-basic-neutral.png")).or_fail()?,
            decode_sprite(include_bytes!("../assets/button-basic-focused.png")).or_fail()?,
            decode_sprite(include_bytes!("../assets/button-basic-pressed.png")).or_fail()?,
        );
        let basic_deep = Button::new(
            basic.pressed.clone(),
            decode_sprite(include_bytes!("../assets/button-basic-deep-focused.png")).or_fail()?,
            decode_sprite(include_bytes!("../assets/button-basic-deep-pressed.png")).or_fail()?,
        );
        Ok(Self { basic, basic_deep })
    }
}

#[derive(Debug)]
pub struct Button {
    pub neutral: Sprite,
    pub focused: Sprite,
    pub pressed: Sprite,
}

impl Button {
    fn new(neutral: Sprite, focused: Sprite, pressed: Sprite) -> Self {
        Self {
            neutral,
            focused,
            pressed,
        }
    }
}

fn load_digits_10x14() -> Result<[Sprite; 10]> {
    let digits = decode_sprite(include_bytes!("../assets/digits-10x14.png")).or_fail()?;
    let base = Size::from_wh(12, 16).to_region();

    fn clip(digits: &Sprite, base: Region, y: i32, x: i32) -> Result<Sprite> {
        let mut region = base.shift_y(y).shift_x(x);
        region.size.width -= 2;
        region.size.height -= 2;
        digits.clip(region).or_fail()
    }

    Ok([
        clip(&digits, base, 0, 0).or_fail()?,
        clip(&digits, base, 0, 1).or_fail()?,
        clip(&digits, base, 0, 2).or_fail()?,
        clip(&digits, base, 0, 3).or_fail()?,
        clip(&digits, base, 0, 4).or_fail()?,
        clip(&digits, base, 1, 0).or_fail()?,
        clip(&digits, base, 1, 1).or_fail()?,
        clip(&digits, base, 1, 2).or_fail()?,
        clip(&digits, base, 1, 3).or_fail()?,
        clip(&digits, base, 1, 4).or_fail()?,
    ])
}
