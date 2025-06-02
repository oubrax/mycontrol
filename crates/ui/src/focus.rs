use std::collections::HashMap;
use gpui::{App, ElementId, FocusHandle, Global, Window};

pub struct FocusRegistry {
    pub handles: HashMap<ElementId, FocusHandle>,
    pub order: Vec<ElementId>,
}

impl FocusRegistry {
    pub fn new() -> Self {
        Self {
            handles: HashMap::new(),
            order: Vec::new(),
        }
    }
}

impl Global for FocusRegistry {}


pub fn init(cx: &mut App) {
    cx.set_global(FocusRegistry::new());
}


/// Register a focusable element with its ElementId key.
/// If an element with the same ID already exists, it will be replaced.
pub fn register_focusable(cx: &mut App, element_id: ElementId, handle: FocusHandle) {
    let registry = cx.global_mut::<FocusRegistry>();

    // If this ElementId doesn't exist in our order, add it
    if !registry.handles.contains_key(&element_id) {
        registry.order.push(element_id.clone());
    }

    // Insert or replace the handle for this ElementId
    registry.handles.insert(element_id, handle);
}

/// Get or create a FocusHandle for the given ElementId.
/// This ensures each ElementId has exactly one unique FocusHandle.
pub fn get_or_create_focus_handle(cx: &mut App, element_id: ElementId) -> FocusHandle {
    // Check if we already have a handle for this ElementId
    {
        let registry = cx.global::<FocusRegistry>();
        if let Some(handle) = registry.handles.get(&element_id) {
            return handle.clone();
        }
    }

    // Create a new handle and register it
    let handle = cx.focus_handle();
    register_focusable(cx, element_id, handle.clone());
    handle
}


pub fn focus_next(window: &mut Window, cx: &mut App) {
    let registry = cx.global::<FocusRegistry>();
    if registry.order.is_empty() {
        println!("handles empty");
        return;
    }

    // Find the currently focused element
    let current_idx = registry.order.iter().position(|element_id| {
        if let Some(handle) = registry.handles.get(element_id) {
            handle.is_focused(window)
        } else {
            false
        }
    });

    // Calculate next index
    let next_idx = match current_idx {
        Some(idx) => (idx + 1) % registry.order.len(),
        None => 0,
    };

    // Focus the next element
    if let Some(element_id) = registry.order.get(next_idx) {
        if let Some(handle) = registry.handles.get(element_id) {
            print!("Focusing element: {:?}", element_id);
            handle.focus(window);
        }
    }
}
