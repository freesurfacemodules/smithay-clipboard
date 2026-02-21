//! DnD (drag-and-drop) event forwarding.
//!
//! smithay-clipboard's data device receives DnD events from the Wayland
//! compositor, but the upstream library only uses clipboard (selection)
//! functionality. This module fills in the DnD handler stubs and exposes
//! events to the host application via a static queue.

use std::sync::Mutex;

/// A DnD event from the Wayland compositor.
#[derive(Debug)]
pub enum DndEvent {
    /// Drag entered the window surface.
    Enter { x: f64, y: f64, mime_types: Vec<String> },
    /// Cursor moved during drag.
    Motion { x: f64, y: f64 },
    /// Drag left the window without dropping.
    Leave,
    /// Drop completed. `data` contains the raw content (e.g. `text/uri-list`).
    Drop { x: f64, y: f64, data: Vec<u8> },
}

static DND_EVENTS: Mutex<Vec<DndEvent>> = Mutex::new(Vec::new());

/// Drain all pending DnD events. Call this periodically from the main thread.
pub fn take_dnd_events() -> Vec<DndEvent> {
    std::mem::take(&mut *DND_EVENTS.lock().unwrap())
}

pub(crate) fn push_event(event: DndEvent) {
    DND_EVENTS.lock().unwrap().push(event);
}
