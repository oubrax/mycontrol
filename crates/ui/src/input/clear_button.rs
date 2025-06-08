use gpui::{App, Hsla, Styled};

use crate::{
    button::{Button, ButtonVariants as _},
    ActiveTheme as _, Icon, IconName, Sizable as _,
};

#[inline]
pub(crate) fn clear_button(muted_foreground: Hsla) -> Button {
    Button::new("clean")
        .icon(Icon::new(IconName::CircleX))
        .ghost()
        .xsmall()
        .text_color(muted_foreground)
}
