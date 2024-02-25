use crate::{app::App, event::Event};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointerEvent {
    pub event_type: EventType,
    pub x: i32,
    pub y: i32,
    pub pointer_id: i32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GestureEvent {
    // Select picker Tool
    Tap,

    // Select selection Tool
    DoubleTap,

    // Move camera
    Swipe,

    // Zoom in / out
    DoubleTapSwipe,
    Pinch,

    // Undo
    TwoFingerTap,

    // Redo
    ThreeFingerTap,
}

#[derive(Debug, Default)]
pub struct GestureRecognizer {}

impl GestureRecognizer {
    pub fn handle_pointer_event(
        &mut self,
        app: &mut App,
        event: PointerEvent,
    ) -> orfail::Result<Event> {
        todo!()
    }
}
