use gpui::{
    div, prelude::FluentBuilder as _, App, Corners, Div, Edges, ElementId, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, Styled, Window,
};
use std::{cell::Cell, rc::Rc};

use crate::{
    button::{Button, ButtonVariant, ButtonVariants},
    Disableable, Sizable, Size,
};

/// A ButtonGroup element, to wrap multiple buttons in a group.
#[derive(IntoElement)]
pub struct ButtonGroup {
    pub base: Div,
    id: ElementId,
    children: Vec<Button>,
    pub(super) multiple: bool,
    pub(super) disabled: bool,

    // The button props
    pub(super) compact: bool,
    pub(super) outline: bool,
    pub(super) variant: Option<ButtonVariant>,
    pub(super) size: Option<Size>,

    // Selection state
    selected_indices: Vec<usize>,
    selected_variant: Option<ButtonVariant>,

    on_click: Option<Rc<dyn Fn(&Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl Disableable for ButtonGroup {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ButtonGroup {
    /// Creates a new ButtonGroup.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            id: id.into(),
            variant: None,
            size: None,
            compact: false,
            outline: false,
            multiple: false,
            disabled: false,
            selected_indices: Vec::new(),
            selected_variant: None,
            on_click: None,
        }
    }

    /// Adds a button as a child to the ButtonGroup.
    pub fn child(mut self, child: Button) -> Self {
        self.children.push(child.disabled(self.disabled));
        self
    }

    /// Adds multiple buttons as children to the ButtonGroup.
    pub fn children(mut self, children: impl IntoIterator<Item = Button>) -> Self {
        self.children.extend(children);
        self
    }

    /// With the multiple selection mode.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// With the compact mode for the ButtonGroup.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// With the outline mode for the ButtonGroup.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// Set the initially selected button indices.
    /// For single selection mode, only the first index will be used.
    pub fn selected(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.selected_indices = indices.into_iter().collect();
        if !self.multiple && self.selected_indices.len() > 1 {
            // In single selection mode, keep only the first selected index
            self.selected_indices.truncate(1);
        }
        self
    }

    /// Set a custom variant for selected buttons.
    /// If not set, selected buttons will use the default selected styling.
    pub fn selected_variant(mut self, variant: ButtonVariant) -> Self {
        self.selected_variant = Some(variant);
        self
    }

    /// Sets the on_click handler for the ButtonGroup.
    ///
    /// The handler first argument is a vector of the selected button indices.
    ///
    /// The `&Vec<usize>` is the indices of the clicked (selected in `multiple` mode) buttons.
    /// For example: `[0, 2, 3]` is means the first, third and fourth buttons are clicked.
    ///
    /// ```rust
    /// ButtonGroup::new("size-button")
    ///    .child(Button::new("large").label("Large").selected(self.size == Size::Large))
    ///    .child(Button::new("medium").label("Medium").selected(self.size == Size::Medium))
    ///    .child(Button::new("small").label("Small").selected(self.size == Size::Small))
    ///    .on_click(cx.listener(|view, clicks: &Vec<usize>, _, cx| {
    ///        if clicks.contains(&0) {
    ///            view.size = Size::Large;
    ///        } else if clicks.contains(&1) {
    ///            view.size = Size::Medium;
    ///        } else if clicks.contains(&2) {
    ///            view.size = Size::Small;
    ///        }
    ///        cx.notify();
    ///    }))
    /// ```
    pub fn on_click(
        mut self,
        handler: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Sizable for ButtonGroup {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ButtonVariants for ButtonGroup {
    fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = Some(variant);
        self
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let children_len = self.children.len();
        // Use the ButtonGroup's selected_indices as the initial state
        let selected_ixs = self.selected_indices.clone();
        let state = Rc::new(Cell::new(None));

        // Create a shared handler for the ButtonGroup's on_click
        let group_on_click = self.on_click.clone();
        let multiple = self.multiple;
        let disabled = self.disabled;

        self.base
            .id(self.id)
            .flex()
            .items_center()
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(child_index, child)| {
                        let state = Rc::clone(&state);
                        let group_handler = group_on_click.clone();
                        let selected_ixs_for_handler = selected_ixs.clone();

                        let child = if children_len == 1 {
                            child
                        } else if child_index == 0 {
                            // First
                            child
                                .border_corners(Corners {
                                    top_left: true,
                                    top_right: false,
                                    bottom_left: true,
                                    bottom_right: false,
                                })
                                .border_edges(Edges {
                                    left: true,
                                    top: true,
                                    right: true,
                                    bottom: true,
                                })
                        } else if child_index == children_len - 1 {
                            // Last
                            child
                                .border_edges(Edges {
                                    left: false,
                                    top: true,
                                    right: true,
                                    bottom: true,
                                })
                                .border_corners(Corners {
                                    top_left: false,
                                    top_right: true,
                                    bottom_left: false,
                                    bottom_right: true,
                                })
                        } else {
                            // Middle
                            child
                                .border_corners(Corners::all(false))
                                .border_edges(Edges {
                                    left: false,
                                    top: true,
                                    right: true,
                                    bottom: true,
                                })
                        }
                        .stop_propagation(false)
                        .when_some(self.size, |this, size| this.with_size(size))
                        .when_some(self.variant, |this, variant| this.with_variant(variant))
                        .when(self.compact, |this| this.compact())
                        .when(self.outline, |this| this.outline());

                        // Add click handler that integrates with ButtonGroup logic and focus system
                        let child = if !disabled {
                            child.on_click(cx, move |_, window, app| {
                                // Set the clicked button index
                                state.set(Some(child_index));

                                // Handle ButtonGroup selection logic
                                if let Some(group_handler) = &group_handler {
                                    let mut updated_selected_ixs = selected_ixs_for_handler.clone();

                                    if multiple {
                                        if let Some(pos) = updated_selected_ixs.iter().position(|&i| i == child_index) {
                                            updated_selected_ixs.remove(pos);
                                        } else {
                                            updated_selected_ixs.push(child_index);
                                        }
                                    } else {
                                        updated_selected_ixs.clear();
                                        updated_selected_ixs.push(child_index);
                                    }

                                    group_handler(&updated_selected_ixs, window, app);
                                }
                            })
                        } else {
                            child
                        };

                        child
                    }),
            )
    }
}