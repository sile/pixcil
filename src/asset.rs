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
    pub alphabet_10x14: [Sprite; 27],
    pub number_box: Sprite,
    pub slider_cursor: Sprite,
    pub right_arrow: Sprite,
    pub hand: Hand,
}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Self {
            icons: Icons::load().or_fail()?,
            buttons: Buttons::load().or_fail()?,
            toggle: Toggle::load().or_fail()?,
            digits_10x14: load_digits_10x14().or_fail()?,
            alphabet_10x14: load_alphabet_10x14().or_fail()?,
            number_box: decode_sprite(include_bytes!("../assets/number-box.png"))
                .or_fail()?
                .clip(Size::from_wh(64, 32).to_region())
                .or_fail()?,
            slider_cursor: decode_sprite(include_bytes!("../assets/slider-cursor.png"))
                .or_fail()?,
            right_arrow: decode_sprite(include_bytes!("../assets/right-arrow.png")).or_fail()?,
            hand: Hand::load().or_fail()?,
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
            IconId::PenStroke => &self.icons.pen_stroke,
            IconId::PenLine => &self.icons.pen_line,
            IconId::PenRectangle => &self.icons.pen_rectangle,
            IconId::PenCircle => &self.icons.pen_circle,
            IconId::Bucket => &self.icons.bucket,
            IconId::ScissorRectangle => &self.icons.scissor_rectangle,
            IconId::ScissorLasso => &self.icons.scissor_lasso,
            IconId::Lasso => &self.icons.lasso,
            IconId::GoLeft => &self.icons.go_left,
            IconId::GoRight => &self.icons.go_right,
            IconId::GoTop => &self.icons.go_top,
            IconId::GoBottom => &self.icons.go_bottom,
            IconId::GoCenter => &self.icons.go_center,
        }
    }

    pub fn get_button(&self, kind: ButtonKind) -> &Button {
        match kind {
            ButtonKind::Basic => &self.buttons.basic,
            ButtonKind::BasicDeep => &self.buttons.basic_deep,
            ButtonKind::BasicPressed => &self.buttons.basic_pressed,
            ButtonKind::SliderLeft => &self.buttons.slider_left,
            ButtonKind::SliderRight => &self.buttons.slider_right,
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
    Draw, // TODO: remove
    Erase,
    Select,
    Pick,
    Move,
    Save,
    Load,
    Import,
    PenStroke,
    PenLine,
    PenRectangle,
    PenCircle,
    Bucket,
    ScissorRectangle,
    ScissorLasso,
    Lasso,
    GoLeft,
    GoRight,
    GoTop,
    GoBottom,
    GoCenter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Basic,
    BasicDeep,
    BasicPressed,
    SliderLeft,
    SliderRight,
}

impl ButtonKind {
    pub fn size(self) -> Size {
        match self {
            ButtonKind::Basic => Size::square(64),
            ButtonKind::BasicDeep => Size::square(64),
            ButtonKind::BasicPressed => Size::square(64),
            ButtonKind::SliderLeft => Size::square(32),
            ButtonKind::SliderRight => Size::square(32),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    text: Vec<Alphabet>,
    margin: u32,
    alphabet_size: Size,
}

impl Text {
    pub const fn new(text: Vec<Alphabet>) -> Self {
        Self {
            text,
            margin: 2,
            alphabet_size: Size::from_wh(10, 14),
        }
    }

    pub fn get(&self) -> &[Alphabet] {
        &self.text
    }

    pub fn margin(&self) -> u32 {
        self.margin
    }

    pub fn size(&self) -> Size {
        let mut size = self.alphabet_size;
        size.width = (self.text.len() as u32 * (self.alphabet_size.width + self.margin))
            .saturating_sub(self.margin);
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
                    ' ' => Alphabet::Space,
                    _ => return Err(Failure::new(format!("unknown alphabet: {c:?}"))),
                })
            })
            .collect::<Result<_>>()
            .map(Self::new)
    }
}

// TODO: rename
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
    Space,
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
    pub pen_stroke: Sprite,
    pub pen_line: Sprite,
    pub pen_rectangle: Sprite,
    pub pen_circle: Sprite,
    pub bucket: Sprite,
    pub scissor_rectangle: Sprite,
    pub scissor_lasso: Sprite,
    pub lasso: Sprite,
    pub go_left: Sprite,
    pub go_right: Sprite,
    pub go_top: Sprite,
    pub go_bottom: Sprite,
    pub go_center: Sprite,
}

impl Icons {
    fn load() -> Result<Self> {
        let go = decode_sprite(include_bytes!("../assets/icon-go.png")).or_fail()?;
        let block = Size::square(64).to_region();
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
            pen_stroke: decode_sprite(include_bytes!("../assets/icon-pen-stroke.png")).or_fail()?,
            pen_line: decode_sprite(include_bytes!("../assets/icon-pen-line.png")).or_fail()?,
            pen_rectangle: decode_sprite(include_bytes!("../assets/icon-pen-rectangle.png"))
                .or_fail()?,
            pen_circle: decode_sprite(include_bytes!("../assets/icon-pen-circle.png")).or_fail()?,
            bucket: decode_sprite(include_bytes!("../assets/icon-bucket.png")).or_fail()?,
            scissor_rectangle: decode_sprite(include_bytes!(
                "../assets/icon-scissor-rectangle.png"
            ))
            .or_fail()?,
            scissor_lasso: decode_sprite(include_bytes!("../assets/icon-scissor-lasso.png"))
                .or_fail()?,
            lasso: decode_sprite(include_bytes!("../assets/icon-lasso.png")).or_fail()?,
            go_left: go.clip(block).or_fail()?,
            go_right: go.clip(block.shift_x(1)).or_fail()?,
            go_top: go.clip(block.shift_x(2)).or_fail()?,
            go_bottom: go.clip(block.shift_x(3)).or_fail()?,
            go_center: go.clip(block.shift_x(4)).or_fail()?,
        })
    }
}

#[derive(Debug)]
pub struct Buttons {
    pub basic: Button,
    pub basic_deep: Button,
    pub basic_pressed: Button,
    pub slider_left: Button,
    pub slider_right: Button,
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
        let basic_pressed = Button::new(
            basic.pressed.clone(),
            basic.pressed.clone(),
            basic.pressed.clone(),
        );

        let slider_buttons =
            decode_sprite(include_bytes!("../assets/slider-button.png")).or_fail()?;
        let slider_button_block = Size::square(32).to_region();
        let slider_left = Button::new(
            slider_buttons.clip(slider_button_block).or_fail()?,
            slider_buttons
                .clip(slider_button_block.shift_x(1))
                .or_fail()?,
            slider_buttons
                .clip(slider_button_block.shift_x(2))
                .or_fail()?,
        );
        let slider_button_block = slider_button_block.shift_y(1);
        let slider_right = Button::new(
            slider_buttons.clip(slider_button_block).or_fail()?,
            slider_buttons
                .clip(slider_button_block.shift_x(1))
                .or_fail()?,
            slider_buttons
                .clip(slider_button_block.shift_x(2))
                .or_fail()?,
        );

        Ok(Self {
            basic,
            basic_deep,
            basic_pressed,
            slider_left,
            slider_right,
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

#[derive(Debug)]
pub struct Hand {
    pub open: Sprite,
    pub close: Sprite,
}

impl Hand {
    fn load() -> Result<Self> {
        let sprite = decode_sprite(include_bytes!("../assets/drag-hands.png")).or_fail()?;
        let block = Region::new(Position::ORIGIN, Size::square(64));
        Ok(Self {
            open: sprite.clip(block).or_fail()?,
            close: sprite.clip(block.shift_x(1)).or_fail()?,
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

fn load_alphabet_10x14() -> Result<[Sprite; 27]> {
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
        clip(&q_z, block, 2, 2).or_fail()?,
    ])
}
