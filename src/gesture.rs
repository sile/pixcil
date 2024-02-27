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
    Pointerover,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

const MOVE_DELTA_THRESHOLD: i32 = 10;

#[derive(Debug, Clone, Copy)]
struct Touch {
    position: Position,
    last_position: Position,
    is_primary: bool,
    moved: bool,
}

impl Touch {
    fn new(event: PointerEvent) -> Self {
        Self {
            position: event.position(),
            last_position: event.position(),
            is_primary: event.is_primary,
            moved: false,
        }
    }

    fn set_position(&mut self, position: Position) {
        let delta = position - self.position;
        if delta.x.abs() < MOVE_DELTA_THRESHOLD || delta.y.abs() < MOVE_DELTA_THRESHOLD {
            return;
        }

        self.last_position = self.position;
        self.position = position;
        self.moved = true;
    }
}

#[derive(Debug, Default)]
pub struct GestureRecognizer {
    touches: HashMap<i32, Touch>,
    pending_gesture: Option<GestureEvent>,
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
                self.touches.insert(pointer.pointer_id, Touch::new(pointer));
                if self.touches.values().all(|touch| !touch.moved) {
                    self.pending_gesture = match self.touches.len() {
                        1 => Some(GestureEvent::Tap),
                        2 => Some(GestureEvent::TwoFingerTap),
                        _ => None,
                    }
                }
            }
            EventType::Pointermove => {
                let n = self.touches.len();
                let Some(touch) = self.touches.get_mut(&pointer.pointer_id) else {
                    return Ok(None);
                };
                touch.set_position(pointer.position());
                if !touch.moved {
                    return Ok(None);
                }
                self.pending_gesture = None;

                if n == 1 {
                    let delta = touch.position - touch.last_position;
                    return Ok(Some(GestureEvent::Swipe { delta }));
                } else if n != 2 {
                    return Ok(None);
                }

                return Ok(self.decide_two_finger_gesture());
            }
            _ => {
                self.touches.remove(&pointer.pointer_id);
                if self.touches.is_empty() {
                    return Ok(self.pending_gesture.take());
                }
            }
        }

        Ok(None)
    }

    fn decide_two_finger_gesture(&self) -> Option<GestureEvent> {
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

        let delta = if t0.is_primary { d0 } else { d1 };
        if d0.x.is_positive() && d1.x.is_positive() {
            Some(GestureEvent::TwoFingerSwipe { delta })
        } else if d0.x.is_negative() && d1.x.is_negative() {
            Some(GestureEvent::TwoFingerSwipe { delta })
        } else if d0.y.is_positive() && d1.y.is_positive() {
            Some(GestureEvent::TwoFingerSwipe { delta })
        } else if d0.y.is_negative() && d1.y.is_negative() {
            Some(GestureEvent::TwoFingerSwipe { delta })
        } else {
            Some(GestureEvent::Pinch { delta })
        }
    }
}
