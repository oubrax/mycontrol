pub mod titlebar;
pub mod pages;

use gpui::prelude::*;
use gpui::{div, AnyView, App, Entity, IntoElement, Render, Window};
use gpui_component::{v_flex, ActiveTheme, Root, TitleBar};

pub struct ControlTitlebar;

impl Render for ControlTitlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new().child("Hello")
    }
}
pub struct ControlRoot {
    pub title_bar: Entity<ControlTitlebar>,
    pub view: AnyView,
}

impl ControlRoot  {
    pub fn new(cx: &mut Context<Self>, view: AnyView) -> Self {
        let title_bar = cx.new(|_| ControlTitlebar);
        Self {
           title_bar, 
            view,
        }
    }
}

impl Render for ControlRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let drawer_layer = Root::render_drawer_layer(window, cx);
        let modal_layer = Root::render_modal_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .bg(cx.theme().background)
            .children(drawer_layer)
            .children(modal_layer)
            .children(notification_layer)
    }
}