use crate::{app::App, event::Event};
use pagurus::{event::MouseEvent, spatial::Position};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointerEvent {
    pub event_type: EventType,
    pub x: i32,
    pub y: i32,
    pub pointer_id: i32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
}

impl PointerEvent {
    pub fn position(&self) -> Position {
        Position::from_xy(self.x, self.y)
    }

    pub fn set_position(&mut self, position: Position) {
        self.x = position.x;
        self.y = position.y;
    }

    pub fn to_mouse_event(self) -> MouseEvent {
        let position = self.position();
        match self.event_type {
            EventType::Pointerdown => MouseEvent::Down { position },
            EventType::Pointermove => MouseEvent::Move { position },
            _ => MouseEvent::Up { position },
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Pointerdown,
    Pointermove,
    Pointerup,
    Pointercancel,
    Pointerout,
    Pointerleave,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GestureEvent {
    // Select picker tool
    Tap,

    // Select selection tool
    TwoFingerTap,

    // Move camera
    Swipe { delta: Position },

    // Undo / Redo
    TwoFingerSwipe { delta: Position },

    // Zoom in / out
    Pinch { delta: Position },
}

const INITIAL_MOVE_THRESHOLD: i32 = 10;

#[derive(Debug, Clone, Copy)]
struct Touch {
    position: Position,
    last_position: Position,
    is_primary: bool,
    moved: bool,
    move_threshold: i32,
}

impl Touch {
    fn new(event: PointerEvent) -> Self {
        Self {
            position: event.position(),
            last_position: event.position(),
            is_primary: event.is_primary,
            moved: false,
            move_threshold: INITIAL_MOVE_THRESHOLD,
        }
    }
}

#[derive(Debug, Default)]
pub struct GestureRecognizer {
    touches: HashMap<i32, Touch>,
    max_touches: usize,
    gesture: Option<GestureEvent>,
}

impl GestureRecognizer {
    pub fn handle_event(
        &mut self,
        _app: &mut App,
        event: &mut Event,
    ) -> orfail::Result<Option<GestureEvent>> {
        if event.is_consumed() {
            return Ok(None);
        }
        let Event::Mouse {
            pointer: Some(pointer),
            ..
        } = *event
        else {
            return Ok(None);
        };
        if !matches!(pointer.pointer_type, PointerType::Touch) {
            return Ok(None);
        }
        // TODO: pen_mode handling

        event.consume();

        match pointer.event_type {
            EventType::Pointerdown => {
                self.handle_pointer_down(pointer);
            }
            EventType::Pointermove => {
                return Ok(self.handle_pointer_move(pointer));
            }
            _ => {
                return Ok(self.handle_pointer_up(pointer));
            }
        }

        Ok(None)
    }

    fn handle_pointer_down(&mut self, pointer: PointerEvent) {
        self.touches.insert(pointer.pointer_id, Touch::new(pointer));
        self.max_touches = self.max_touches.max(self.touches.len());
    }

    fn handle_pointer_move(&mut self, pointer: PointerEvent) -> Option<GestureEvent> {
        let n = self.touches.len();
        let Some(touch) = self.touches.get(&pointer.pointer_id).copied() else {
            return None;
        };

        if n == 1 {
            self.handle_one_finger_move(pointer, touch);
            return self.gesture;
        } else if n != 2 {
            return None;
        }

        self.decide_two_finger_gesture()
    }

    fn handle_one_finger_move(&mut self, pointer: PointerEvent, mut touch: Touch) {
        let delta = pointer.position() - touch.position;
        match self.gesture {
            None => {
                let threshold = 10;
                if delta.x.abs().max(delta.y.abs()) < threshold {
                    return;
                }

                touch.position = pointer.position();
                self.touches.insert(pointer.pointer_id, touch);
                self.gesture = Some(GestureEvent::Swipe { delta });
            }
            Some(GestureEvent::Swipe { .. }) => {
                touch.position = pointer.position();
                self.touches.insert(pointer.pointer_id, touch);
                self.gesture = Some(GestureEvent::Swipe { delta });
            }
            _ => {}
        }
    }

    fn handle_pointer_up(&mut self, pointer: PointerEvent) -> Option<GestureEvent> {
        if self.touches.remove(&pointer.pointer_id).is_none() {
            return None;
        }
        if !self.touches.is_empty() {
            return None;
        }

        let gesture = self.gesture.take();
        let max_touches = self.max_touches;
        self.max_touches = 0;

        if gesture.is_some() {
            return None;
        }

        match max_touches {
            1 => Some(GestureEvent::Tap),
            2 => Some(GestureEvent::TwoFingerTap),
            _ => None,
        }
    }

    fn decide_two_finger_gesture(&mut self) -> Option<GestureEvent> {
        if self.touches.len() != 2 {
            return None;
        }

        let t0 = self.touches.values().next().copied()?;
        let t1 = self.touches.values().nth(1).copied()?;
        if !(t0.moved && t1.moved) {
            return None;
        }

        let d0 = t0.position - t0.last_position;
        let d1 = t1.position - t1.last_position;

        self.gesture = if d0.x.is_positive() && d1.x.is_positive()
            || d0.x.is_negative() && d1.x.is_negative()
            || d0.y.is_positive() && d1.y.is_positive()
            || d0.y.is_negative() && d1.y.is_negative()
        {
            if self.gesture.is_some() {
                return None;
            }

            let delta = if t0.is_primary { d0 } else { d1 };
            Some(GestureEvent::TwoFingerSwipe { delta })
        } else {
            if !matches!(self.gesture, None | Some(GestureEvent::Pinch { .. })) {
                return None;
            }

            for t in self.touches.values_mut() {
                t.move_threshold = INITIAL_MOVE_THRESHOLD * 2;
            }

            let mut d0 = t0.last_position - t1.last_position;
            let mut d1 = t0.position - t1.position;
            d0.x = d0.x.abs();
            d0.y = d0.y.abs();
            d1.x = d1.x.abs();
            d1.y = d1.y.abs();
            let delta = d1 - d0;
            Some(GestureEvent::Pinch { delta })
        };
        self.gesture
    }
}
