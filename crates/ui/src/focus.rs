use gpui::{App, ClickEvent, ElementId, FocusHandle, Global, Window};
use std::collections::{HashMap, HashSet};

type ButtonClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

pub struct FocusRegistry {
    pub handles: HashMap<ElementId, FocusHandle>,
    pub order: Vec<ElementId>,
    pub button_handlers: HashMap<ElementId, ButtonClickHandler>,
    pub disabled: HashSet<ElementId>,
}

impl FocusRegistry {
    pub fn new() -> Self {
        Self {
            handles: HashMap::new(),
            order: Vec::new(),
            button_handlers: HashMap::new(),
            disabled: HashSet::new(),
        }
    }
}

impl Global for FocusRegistry {}

pub fn init(cx: &mut App) {
    cx.set_global(FocusRegistry::new());
}

/// Register a button's click handler
pub fn register_button_handler(cx: &mut App, element_id: ElementId, handler: ButtonClickHandler) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.button_handlers.insert(element_id, handler);
}

pub fn remove_focus(cx: &mut App, element_id: ElementId) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.button_handlers.remove(&element_id);
    registry.handles.remove(&element_id);
    registry.order.retain(|id| id != &element_id);
    registry.disabled.remove(&element_id);
}

/// Handle the enter focus event with window context - this is the proper implementation
pub fn handle_enter_focus_event_with_window(window: &mut Window, app: &mut App) {
    // Find the currently focused element
    let focused_element_id = {
        let registry = app.global::<FocusRegistry>();
        registry
            .order
            .iter()
            .find(|element_id| {
                if let Some(handle) = registry.handles.get(element_id) {
                    handle.is_focused(window)
                } else {
                    false
                }
            })
            .cloned()
    };

    if let Some(element_id) = focused_element_id {
        // Extract the handler from the registry to avoid borrowing conflicts
        let handler = {
            let registry = app.global_mut::<FocusRegistry>();
            registry.button_handlers.remove(&element_id)
        };

        if let Some(handler) = handler {
            // Create a default click event and trigger the handler
            let click_event = ClickEvent::default();

            // Call the handler
            handler(&click_event, window, app);

            // Put the handler back in the registry
            let registry = app.global_mut::<FocusRegistry>();
            registry.button_handlers.insert(element_id, handler);
        }
    }
}

/// Register a focusable element with its ElementId key.
/// If an element with the same ID already exists, it will be replaced.
pub fn register_focusable(cx: &mut App, element_id: ElementId, handle: FocusHandle) {
    let registry = cx.global_mut::<FocusRegistry>();
    // Order is now managed strictly via set_focus_cycle().
    // Just register the handle for ElementId. No mutation of order permitted here.
    registry.handles.insert(element_id, handle);
}

/*
 * Focus cycle functions based on a fully-custom cycle order.
 */

/// Replace the focus cycle order. Only elements in the cycle will be included in tab traversal.
/// Any element in the cycle missing a handle will be ignored for tab purposes until registered.
/// This function replaces the previous focus order implementation.
pub fn set_focus_cycle<I: Into<ElementId>>(cx: &mut App, ids: Vec<I>) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.order.clear();
    // Convert all to ElementId, allow repeats (user responsibility to not do so).
    registry.order.extend(ids.into_iter().map(Into::into));
}

/// Get or create a FocusHandle for the given ElementId.
/// This ensures each ElementId has exactly one unique FocusHandle.
/// Calling this does NOT affect tab ordering/cycle except that,
/// if an element is not in the current focus cycle, it will not participate in traversal.
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

/// Disable a list of focus handles.
/// Disabled handles will be skipped during tab traversal.
pub fn disable_focus_handles(cx: &mut App, ids: Vec<ElementId>) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.disabled.extend(ids);
}

/// Enable a list of focus handles.
pub fn enable_focus_handles(cx: &mut App, ids: Vec<ElementId>) {
    let registry = cx.global_mut::<FocusRegistry>();
    for id in ids {
        registry.disabled.remove(&id);
    }
}

/// Clear all disabled focus handles.
pub fn clear_disabled_focus_handles(cx: &mut App) {
    let registry = cx.global_mut::<FocusRegistry>();
    registry.disabled.clear();
}

pub fn focus_next(window: &mut Window, cx: &mut App) {
    let registry = cx.global::<FocusRegistry>();
    if registry.order.is_empty() {
        return;
    }

    let focused_idx = registry.order.iter().position(|element_id| {
        if let Some(handle) = registry.handles.get(element_id) {
            handle.is_focused(window)
        } else {
            false
        }
    });

    let start_idx = focused_idx.map_or(0, |idx| idx + 1);

    if let Some(element_id) = registry
        .order
        .iter()
        .cycle()
        .skip(start_idx)
        .take(registry.order.len())
        .find(|id| !registry.disabled.contains(id) && registry.handles.contains_key(id))
        .cloned()
    {
        if let Some(handle) = registry.handles.get(&element_id) {
            handle.focus(window);
        }
    }
}

pub fn focus_previous(window: &mut Window, cx: &mut App) {
    let registry = cx.global::<FocusRegistry>();
    if registry.order.is_empty() {
        return;
    }

    let focused_idx = registry.order.iter().position(|element_id| {
        if let Some(handle) = registry.handles.get(element_id) {
            handle.is_focused(window)
        } else {
            false
        }
    });

    let start_idx = focused_idx.unwrap_or(0);

    if let Some(element_id) = registry
        .order
        .iter()
        .rev()
        .cycle()
        .skip(registry.order.len() - start_idx)
        .take(registry.order.len())
        .find(|id| !registry.disabled.contains(id) && registry.handles.contains_key(id))
        .cloned()
    {
        if let Some(handle) = registry.handles.get(&element_id) {
            handle.focus(window);
        }
    }
}

/// Unfocus all elements by blurring the window
pub fn unfocus_all(window: &mut Window) {
    window.blur();
}

#[derive(Clone, Copy, Debug)]
pub struct EnterFocusEvent {}
