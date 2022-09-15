use pagurus::event::{Event as PagurusEvent, MouseEvent};
use pagurus::input::MouseButton;
use pagurus::{spatial::Position, ActionId};

#[derive(Debug)]
pub enum Event {
    Timeout(ActionId),
    Mouse {
        action: MouseAction,
        position: Position,
        consumed: bool,
    },
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

    pub fn is_consumed(&self) -> bool {
        matches!(self, Self::Mouse { consumed: true, .. })
    }

    pub fn from_pagurus_event(event: PagurusEvent) -> Option<Self> {
        match event {
            PagurusEvent::Timeout(e) => Some(Self::Timeout(e.id)),
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
