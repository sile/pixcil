use crate::gesture::{GestureEvent, PointerEvent};
use pagurus::event::{Event as PagurusEvent, MouseEvent, TimeoutTag};
use pagurus::image::Sprite;
use pagurus::spatial::Position;
use pagurus::spatial::{Contains, Region};

#[derive(Debug)]
pub enum Event {
    Timeout(TimeoutTag),
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
        pointer: Option<PointerEvent>,
    },
    Gesture(GestureEvent),
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

    pub fn from_pagurus_event(event: PagurusEvent) -> Option<Self> {
        match event {
            PagurusEvent::Timeout(e) => Some(Self::Timeout(e)),
            PagurusEvent::Mouse(e) => match e {
                MouseEvent::Move { position } => Some(Self::Mouse {
                    action: MouseAction::Move,
                    position,
                    consumed: false,
                    pointer: None,
                }),
                MouseEvent::Down { position } => Some(Self::Mouse {
                    action: MouseAction::Down,
                    position,
                    consumed: false,
                    pointer: None,
                }),
                MouseEvent::Up { position } => Some(Self::Mouse {
                    action: MouseAction::Up,
                    position,
                    consumed: false,
                    pointer: None,
                }),
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct InputId(u64);

impl InputId {
    pub fn get_and_increment(&mut self) -> Self {
        let id = *self;
        self.0 += 1;
        id
    }
}
