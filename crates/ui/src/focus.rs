use std::collections::HashMap;
use gpui::{AnyEntity, App, ClickEvent, ElementId, FocusHandle, Global, Window};

type ButtonClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

pub struct FocusRegistry {
    pub handles: HashMap<ElementId, FocusHandle>,
    pub order: Vec<ElementId>,
    pub main_entity: AnyEntity,
    pub button_handlers: HashMap<ElementId, ButtonClickHandler>,
}

impl FocusRegistry {
    pub fn new(entity: AnyEntity) -> Self {
        Self {
            handles: HashMap::new(),
            order: Vec::new(),
            main_entity: entity,
            button_handlers: HashMap::new(),
        }
    }
}

impl Global for FocusRegistry {}

pub fn init(cx: &mut App, entity: AnyEntity) {
    cx.set_global(FocusRegistry::new(entity));
}

/// Register a button's click handler
pub fn register_button_handler(
    cx: &mut App,
    element_id: ElementId,
    handler: ButtonClickHandler,
) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.button_handlers.insert(element_id, handler);
}

/// Handle the enter focus event with window context - this is the proper implementation
pub fn handle_enter_focus_event_with_window(window: &mut Window, app: &mut App) {
    // Find the currently focused element
    let focused_element_id = {
        let registry = app.global::<FocusRegistry>();
        registry.order.iter().find(|element_id| {
            if let Some(handle) = registry.handles.get(element_id) {
                handle.is_focused(window)
            } else {
                false
            }
        }).cloned()
    };

    if let Some(element_id) = focused_element_id {
        println!("Found focused button: {:?}", element_id);

        // Extract the handler from the registry to avoid borrowing conflicts
        let handler = {
            let mut registry = app.global_mut::<FocusRegistry>();
            registry.button_handlers.remove(&element_id)
        };

        if let Some(handler) = handler {
            // Create a default click event and trigger the handler
            let click_event = ClickEvent::default();
            println!("Triggering click handler for button: {:?}", element_id);

            // Call the handler
            handler(&click_event, window, app);

            // Put the handler back in the registry
            let mut registry = app.global_mut::<FocusRegistry>();
            registry.button_handlers.insert(element_id, handler);
        } else {
            println!("No click handler found for button: {:?}", element_id);
        }
    } else {
        println!("No focused button found");
    }
}

// Removed subscription-based handling - now using direct keystroke observer


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

pub fn main_entity(cx: &mut App) -> AnyEntity {
    let registry = cx.global::<FocusRegistry>();
    registry.main_entity.clone()
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
            println!("Focusing next element: {:?}", element_id);
            handle.focus(window);
        }
    }
}

pub fn focus_previous(window: &mut Window, cx: &mut App) {
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

    // Calculate previous index
    let prev_idx = match current_idx {
        Some(idx) => {
            if idx == 0 {
                registry.order.len() - 1 // Wrap to last element
            } else {
                idx - 1
            }
        }
        None => registry.order.len() - 1, // If nothing focused, go to last element
    };

    // Focus the previous element
    if let Some(element_id) = registry.order.get(prev_idx) {
        if let Some(handle) = registry.handles.get(element_id) {
            println!("Focusing previous element: {:?}", element_id);
            handle.focus(window);
        }
    }
}

/// Unfocus all elements by blurring the window
pub fn unfocus_all(window: &mut Window) {
    println!("Unfocusing all elements");
    window.blur();
}


#[derive(Clone, Copy, Debug)]
pub struct EnterFocusEvent {}