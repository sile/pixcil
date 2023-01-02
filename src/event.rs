use crate::app::App;
use pagurus::event::{Event as PagurusEvent, MouseEvent};
use pagurus::image::Sprite;
use pagurus::input::MouseButton;
use pagurus::spatial::{Contains, Region};
use pagurus::{spatial::Position, ActionId};

#[derive(Debug)]
pub enum Event {
    Timeout(TimeoutId),
    Import {
        image: Sprite,
    },
    Input {
        id: InputId,
        text: String,
    },
    Mouse {
        action: MouseAction,
        position: Position,
        consumed: bool,
    },
    Noop, // TODO: rename
}

impl Event {
    pub fn position(&self) -> Option<Position> {
        if let Self::Mouse { position, .. } = self {
            Some(*position)
        } else {
            None
        }
    }

    pub fn consume(&mut self) {
        if let Self::Mouse { consumed, .. } = self {
            *consumed = true;
        }
    }

    pub fn consume_if_contained(&mut self, region: Region) {
        match self {
            Self::Mouse {
                consumed: false,
                position,
                ..
            } if region.contains(position) => {
                self.consume();
            }
            _ => {}
        }
    }

    pub fn is_consumed(&self) -> bool {
        matches!(self, Self::Mouse { consumed: true, .. })
    }

    pub fn from_pagurus_event(app: &mut App, event: PagurusEvent) -> Option<Self> {
        match event {
            PagurusEvent::Timeout(e) => app.take_timeout_id(e.id).map(Self::Timeout),
            PagurusEvent::Mouse(e) => match e {
                MouseEvent::Move { position } => Some(Self::Mouse {
                    action: MouseAction::Move,
                    position,
                    consumed: false,
                }),
                MouseEvent::Down {
                    position,
                    button: MouseButton::Left,
                } => Some(Self::Mouse {
                    action: MouseAction::Down,
                    position,
                    consumed: false,
                }),
                MouseEvent::Up {
                    position,
                    button: MouseButton::Left,
                } => Some(Self::Mouse {
                    action: MouseAction::Up,
                    position,
                    consumed: false,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    Up,
    Down,
    Move,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeoutId(ActionId);

impl TimeoutId {
    pub fn get_and_increment(&mut self) -> Self {
        let id = *self;
        self.0.increment();
        id
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct InputId(ActionId);

impl InputId {
    pub fn get_and_increment(&mut self) -> Self {
        let id = *self;
        self.0.increment();
        id
    }
}
