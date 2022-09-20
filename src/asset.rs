use pagurus::failure::Failure;
use pagurus::spatial::{Position, Region, Size};
use pagurus::{failure::OrFail, Result};
use pagurus_game_std::image::Sprite;
use pagurus_game_std::png::decode_sprite;

#[derive(Debug)]
pub struct Assets {
    pub icons: Icons,
    pub buttons: Buttons,
    pub toggle: Toggle,
    pub digits_10x14: [Sprite; 10],
    pub alphabet_10x14: [Sprite; 26],
}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Self {
            icons: Icons::load().or_fail()?,
            buttons: Buttons::load().or_fail()?,
            toggle: Toggle::load().or_fail()?,
            digits_10x14: load_digits_10x14().or_fail()?,
            alphabet_10x14: load_alphabet_10x14().or_fail()?,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text(Vec<Alphabet>);

impl Text {
    pub const fn new(text: Vec<Alphabet>) -> Self {
        Self(text)
    }

    pub fn get(&self) -> &[Alphabet] {
        &self.0
    }

    pub fn size(&self, margin: u32, alphabet_size: Size) -> Size {
        let mut size = alphabet_size;
        size.width = (self.0.len() as u32 * (alphabet_size.width + margin)).saturating_sub(margin);
        size
    }
}

impl std::str::FromStr for Text {
    type Err = Failure;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| {
                Ok(match c {
                    'A' => Alphabet::A,
                    'B' => Alphabet::B,
                    'C' => Alphabet::C,
                    'D' => Alphabet::D,
                    'E' => Alphabet::E,
                    'F' => Alphabet::F,
                    'G' => Alphabet::G,
                    'H' => Alphabet::H,
                    'I' => Alphabet::I,
                    'J' => Alphabet::J,
                    'K' => Alphabet::K,
                    'L' => Alphabet::L,
                    'M' => Alphabet::M,
                    'N' => Alphabet::N,
                    'O' => Alphabet::O,
                    'P' => Alphabet::P,
                    'Q' => Alphabet::Q,
                    'R' => Alphabet::R,
                    'S' => Alphabet::S,
                    'T' => Alphabet::T,
                    'U' => Alphabet::U,
                    'V' => Alphabet::V,
                    'W' => Alphabet::W,
                    'X' => Alphabet::X,
                    'Y' => Alphabet::Y,
                    'Z' => Alphabet::Z,
                    _ => return Err(Failure::new(format!("unknown alphabet: {c:?}"))),
                })
            })
            .collect::<Result<_>>()
            .map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
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

#[derive(Debug)]
pub struct Toggle {
    pub on_neutral: Sprite,
    pub on_focused: Sprite,
    pub off_neutral: Sprite,
    pub off_focused: Sprite,
}

impl Toggle {
    fn load() -> Result<Self> {
        let on = decode_sprite(include_bytes!("../assets/toggle-on.png")).or_fail()?;
        let off = decode_sprite(include_bytes!("../assets/toggle-off.png")).or_fail()?;
        let block = Region::new(Position::ORIGIN, Size::from_wh(64, 32));
        Ok(Self {
            on_neutral: on.clip(block).or_fail()?,
            on_focused: on.clip(block.shift_y(1)).or_fail()?,
            off_neutral: off.clip(block).or_fail()?,
            off_focused: off.clip(block.shift_y(1)).or_fail()?,
        })
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

fn load_alphabet_10x14() -> Result<[Sprite; 26]> {
    let a_p = decode_sprite(include_bytes!("../assets/A-P-10x14.png")).or_fail()?;
    let q_z = decode_sprite(include_bytes!("../assets/Q-Z-10x14.png")).or_fail()?;
    let block = Size::square(16).to_region();

    fn clip(alphabet: &Sprite, block: Region, y: i32, x: i32) -> Result<Sprite> {
        let mut region = block.shift_y(y).shift_x(x);
        region.size.width = 10;
        region.size.height = 14;
        alphabet.clip(region).or_fail()
    }

    Ok([
        clip(&a_p, block, 0, 0).or_fail()?,
        clip(&a_p, block, 0, 1).or_fail()?,
        clip(&a_p, block, 0, 2).or_fail()?,
        clip(&a_p, block, 0, 3).or_fail()?,
        clip(&a_p, block, 1, 0).or_fail()?,
        clip(&a_p, block, 1, 1).or_fail()?,
        clip(&a_p, block, 1, 2).or_fail()?,
        clip(&a_p, block, 1, 3).or_fail()?,
        clip(&a_p, block, 2, 0).or_fail()?,
        clip(&a_p, block, 2, 1).or_fail()?,
        clip(&a_p, block, 2, 2).or_fail()?,
        clip(&a_p, block, 2, 3).or_fail()?,
        clip(&a_p, block, 3, 0).or_fail()?,
        clip(&a_p, block, 3, 1).or_fail()?,
        clip(&a_p, block, 3, 2).or_fail()?,
        clip(&a_p, block, 3, 3).or_fail()?,
        clip(&q_z, block, 0, 0).or_fail()?,
        clip(&q_z, block, 0, 1).or_fail()?,
        clip(&q_z, block, 0, 2).or_fail()?,
        clip(&q_z, block, 0, 3).or_fail()?,
        clip(&q_z, block, 1, 0).or_fail()?,
        clip(&q_z, block, 1, 1).or_fail()?,
        clip(&q_z, block, 1, 2).or_fail()?,
        clip(&q_z, block, 1, 3).or_fail()?,
        clip(&q_z, block, 2, 0).or_fail()?,
        clip(&q_z, block, 2, 1).or_fail()?,
    ])
}
