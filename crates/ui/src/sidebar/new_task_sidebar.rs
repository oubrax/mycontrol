use crate::{
    Collapsible, Icon, IconName,
    button::{Button, ButtonVariants},
    v_flex,
};
use gpui::{
    App, ClickEvent, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct NewTaskSidebar {
    collapsed: bool,
    on_new_task: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl NewTaskSidebar {
    pub fn new() -> Self {
        Self {
            collapsed: false,
            on_new_task: None,
        }
    }

    pub fn on_new_task(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_new_task = Some(Rc::new(handler));
        self
    }
}

impl Collapsible for NewTaskSidebar {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for NewTaskSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let on_new_task = self.on_new_task.clone();
        v_flex().gap_2().p_2().child(
            div().w_full().child(
                Button::new("new-task")
                    .primary()
                    .icon(Icon::new(IconName::Plus).size_4()) // Bigger icon
                    .when(!self.collapsed, |btn| btn.label("New Task")) // Removed .small() for bigger button
                    .when(self.collapsed, |btn| btn.compact().justify_center()) // Removed .small()
                    .when_some(on_new_task, |btn, handler| {
                        btn.on_click(cx, move |ev, window, cx| {
                            handler(ev, window, cx);
                        })
                    }),
            ),
        )
    }
}
