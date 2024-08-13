use crate::{app::App, event::Event};
use pagurus::{event::MouseEvent, spatial::Position};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointerEvent {
    pub event_type: EventType,
    #[serde(flatten)]
    pub position: Position,
    pub pointer_id: i32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
}

impl PointerEvent {
    pub fn to_mouse_event(self) -> MouseEvent {
        let position = self.position;
        match self.event_type {
            EventType::Pointerdown => MouseEvent::Down { position },
            EventType::Pointermove => MouseEvent::Move { position },
            _ => MouseEvent::Up { position },
        }
    }

    pub fn is_duplicate(self, other: Self) -> bool {
        self.event_type.is_up_like()
            && other.event_type.is_up_like()
            && self.pointer_id == other.pointer_id
            && self.position == other.position
            && self.pointer_type == other.pointer_type
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

impl EventType {
    pub fn is_up_like(self) -> bool {
        matches!(
            self,
            EventType::Pointerup
                | EventType::Pointercancel
                | EventType::Pointerout
                | EventType::Pointerleave
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    // Select bucket tool
    ThreeFingerTap,

    // Move camera
    Swipe { delta: Position },

    // Undo / Redo
    TwoFingerSwipe { undo: bool },

    // Zoom in / out
    Pinch { zoom_in: bool },
}

#[derive(Debug, Clone, Copy)]
struct Touch {
    base_position: Position,
    last_position: Position,
}

impl Touch {
    fn new(position: Position) -> Self {
        Self {
            base_position: position,
            last_position: position,
        }
    }

    fn delta(&self) -> Position {
        self.last_position - self.base_position
    }

    fn reset_base_position(&mut self) {
        self.base_position = self.last_position;
    }
}

fn magnitude(p: Position) -> u32 {
    ((p.x.pow(2) + p.y.pow(2)) as f32).sqrt() as u32
}

#[derive(Debug, Default)]
pub struct GestureRecognizer {
    touches: HashMap<i32, Touch>,
    max_touches: usize,
    gesture: Option<GestureEvent>,
}

impl GestureRecognizer {
    pub fn has_active_touches(&self) -> bool {
        !self.touches.is_empty()
    }

    pub fn handle_event(
        &mut self,
        app: &mut App,
        event: &mut Event,
    ) -> orfail::Result<Option<GestureEvent>> {
        if event.is_consumed() || !app.models().config.gesture {
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
        self.touches
            .insert(pointer.pointer_id, Touch::new(pointer.position));
        self.max_touches = self.max_touches.max(self.touches.len());
    }

    fn handle_pointer_move(&mut self, pointer: PointerEvent) -> Option<GestureEvent> {
        let touch = self.touches.get_mut(&pointer.pointer_id)?;
        touch.last_position = pointer.position;

        let n = self.touches.len();
        if n == 1 {
            self.handle_one_finger_move()
        } else if n == 2 {
            self.handle_two_fingers_move()
        } else {
            None
        }
    }

    fn handle_pointer_up(&mut self, pointer: PointerEvent) -> Option<GestureEvent> {
        self.touches.remove(&pointer.pointer_id)?;
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
            3 => Some(GestureEvent::ThreeFingerTap),
            _ => None,
        }
    }

    fn handle_one_finger_move(&mut self) -> Option<GestureEvent> {
        if self.max_touches != 1 {
            return None;
        }

        let touch = self.touches.values_mut().next()?;
        let delta = touch.delta();
        match self.gesture {
            None => {
                let threshold = 10;
                if magnitude(delta) < threshold {
                    return None;
                }
            }
            Some(GestureEvent::Swipe { .. }) => {}
            _ => {
                return None;
            }
        }
        touch.reset_base_position();
        self.gesture = Some(GestureEvent::Swipe { delta });
        self.gesture
    }

    fn handle_two_fingers_move(&mut self) -> Option<GestureEvent> {
        if self.max_touches != 2 {
            return None;
        }

        match self.gesture {
            None => self.detect_two_fingers_gesture(),
            Some(GestureEvent::Pinch { .. }) => self.handle_pinch(),
            Some(GestureEvent::TwoFingerSwipe { .. }) => None,
            _ => None,
        }
    }

    fn detect_two_fingers_gesture(&mut self) -> Option<GestureEvent> {
        let t0 = self.touches.values().next().copied()?;
        let t1 = self.touches.values().nth(1).copied()?;
        let d0 = t0.delta();
        let d1 = t1.delta();

        if (d0.x.is_positive() && d1.x.is_positive()) || (d0.x.is_negative() && d1.x.is_negative())
        {
            let threshold = 100;
            if d0.x.abs() < threshold || d1.x.abs() < threshold {
                return None;
            }

            let undo = d0.x.is_negative();
            self.gesture = Some(GestureEvent::TwoFingerSwipe { undo });
        } else if (d0.x.is_positive() && d1.x.is_negative())
            || (d0.x.is_negative() && d1.x.is_positive())
            || (d0.y.is_positive() && d1.y.is_negative())
            || (d0.y.is_negative() && d1.y.is_positive())
        {
            let threshold = 40;
            if magnitude(d0) < threshold && magnitude(d1) < threshold {
                return None;
            }

            let m0 = magnitude(t0.base_position - t1.base_position);
            let m1 = magnitude(t0.last_position - t1.last_position);
            if m0 < m1 {
                self.gesture = Some(GestureEvent::Pinch { zoom_in: true });
            } else {
                self.gesture = Some(GestureEvent::Pinch { zoom_in: false });
            }
            for t in self.touches.values_mut() {
                t.reset_base_position();
            }
        }

        self.gesture
    }

    fn handle_pinch(&mut self) -> Option<GestureEvent> {
        let t0 = self.touches.values().next().copied()?;
        let t1 = self.touches.values().nth(1).copied()?;
        let d0 = t0.delta();
        let d1 = t1.delta();

        let threshold = 20;
        if magnitude(d0) < threshold && magnitude(d1) < threshold {
            return None;
        }

        let m0 = magnitude(t0.base_position - t1.base_position);
        let m1 = magnitude(t0.last_position - t1.last_position);
        if m0 < m1 {
            self.gesture = Some(GestureEvent::Pinch { zoom_in: true });
        } else {
            self.gesture = Some(GestureEvent::Pinch { zoom_in: false });
        }
        for t in self.touches.values_mut() {
            t.reset_base_position();
        }
        self.gesture
    }
}
