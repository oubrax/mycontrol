use gpui::{
    div, percentage, px, BoxShadow, Context, IntoElement, ParentElement, Point, Render, Styled,
    WeakEntity, Window,
};
use gpui_component::ActiveTheme;
pub struct AskPage {}

impl AskPage {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {}
    }
}
impl Render for AskPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().justify_center().items_center().flex()
    }
}
