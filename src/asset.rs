use pagurus::{failure::OrFail, Result};
use pagurus_game_std::image::Sprite;
use pagurus_game_std::png::decode_sprite;

#[derive(Debug)]
pub struct Assets {
    pub icons: Icons,
    pub buttons: Buttons,
}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Self {
            icons: Icons::load().or_fail()?,
            buttons: Buttons::load().or_fail()?,
        })
    }

    pub fn get_icon(&self, id: IconId) -> &Sprite {
        match id {
            IconId::Undo => &self.icons.undo,
            IconId::Redo => &self.icons.redo,
        }
    }

    pub fn get_button(&self, kind: ButtonKind) -> &Button {
        match kind {
            ButtonKind::Basic => &self.buttons.basic,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IconId {
    Undo,
    Redo,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonKind {
    Basic,
}

#[derive(Debug)]
pub struct Icons {
    pub undo: Sprite,
    pub redo: Sprite,
}

impl Icons {
    fn load() -> Result<Self> {
        Ok(Self {
            undo: decode_sprite(include_bytes!("../assets/icon-undo.png")).or_fail()?,
            redo: decode_sprite(include_bytes!("../assets/icon-redo.png")).or_fail()?,
        })
    }
}

#[derive(Debug)]
pub struct Buttons {
    pub basic: Button,
}

impl Buttons {
    fn load() -> Result<Self> {
        Ok(Self {
            basic: Button::new(
                decode_sprite(include_bytes!("../assets/button-basic-neutral.png")).or_fail()?,
                decode_sprite(include_bytes!("../assets/button-basic-focused.png")).or_fail()?,
                decode_sprite(include_bytes!("../assets/button-basic-pressed.png")).or_fail()?,
            ),
        })
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
