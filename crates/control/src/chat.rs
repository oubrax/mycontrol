use crate::state::{self, CONTEXT, ChatState, DownMessage, Message, Part, Role, UpMessage};
use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, Context, Div, Entity, EventEmitter, FocusHandle,
    Focusable, KeyDownEvent, Keystroke, ListState, Render, SharedString, Stateful, Styled,
    WeakEntity, Window, div, list, prelude::*, px, rems,
};
use ui::{
    ActiveTheme, Button, ButtonVariants, ContextModal, Disableable, Icon, IconName, Sizable,
    StyledExt, focus, h_flex,
    input::{InputEvent, InputState, TextInput},
    notification::Notification,
    v_flex,
};

#[derive(IntoElement)]
pub struct MessageBubble {
    base: Stateful<Div>,
    message: Message,
    edit_message_view: Option<AnyElement>,
    on_edit_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_cancel_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_submit_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_copy_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    focused: bool,
}

impl MessageBubble {
    pub fn new(msg: Message) -> Self {
        Self {
            base: div().id("message"),
            message: msg,
            edit_message_view: None,
            on_edit_click: None,
            on_cancel_click: None,
            on_submit_click: None,
            on_copy_click: None,
            focused: true,
        }
    }

    fn render_part(part: &Part, _window: &mut Window, _cx: &mut App) -> AnyElement {
        match part {
            Part::Text(t) => SharedString::from(t).into_any_element(),
            Part::ToolCall(_t) => unreachable!(),
        }
        .into_any_element()
    }

    fn edit_message_view(mut self, view: Option<AnyElement>) -> Self {
        self.edit_message_view = view;

        self
    }

    fn on_edit_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_edit_click = Some(Box::new(handler));

        self
    }

    fn on_cancel_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel_click = Some(Box::new(handler));
        self
    }

    fn on_submit_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_submit_click = Some(Box::new(handler));
        self
    }

    fn on_copy_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_copy_click = Some(Box::new(handler));
        self
    }

    fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;

        self
    }
}

impl RenderOnce for MessageBubble {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_textarea = self.edit_message_view.is_some();

        self.base
            .size_full()
            .flex()
            .flex_col()
            .items_end()
            .pb(px(5.))
            .group("message")
            .debug_below()
            .child(
                // Main message container
                div()
                    .id(SharedString::from(format!(
                        "message-bubble-{}",
                        self.message.id
                    )))
                    .flex()
                    .flex_col()
                    .items_end()
                    .gap(px(5.))
                    .w_full()
                    .max_w(rems(46.))
                    .child(
                        // Message content bubble
                        div()
                            .flex()
                            .flex_col()
                            .py(px(8.))
                            .px(if has_textarea { px(5.) } else { px(14.) })
                            .whitespace_normal()
                            .rounded(cx.theme().radius * 1.3)
                            .rounded_br(cx.theme().radius * 0.6)
                            .border_1()
                            .border_color(cx.theme().border)
                            .bg(cx.theme().muted.opacity(0.3))
                            .when(self.focused, |this| {
                                this.border_2().border_color(cx.theme().ring)
                            })
                            .when(!has_textarea, |this| {
                                this.group_hover("message", |this| {
                                    this.bg(cx.theme().muted.opacity(0.8))
                                })
                                .gap(px(10.))
                                .children(
                                    self.message
                                        .parts
                                        .iter()
                                        .map(|p| Self::render_part(p, window, cx)),
                                )
                            })
                            .when(has_textarea, |this| this.w_full())
                            .when_some(self.edit_message_view, |this, view| {
                                this.child(view).w_full()
                            })
                            .when(has_textarea, |this| {
                                this.child(
                                    // Edit mode controls
                                    h_flex()
                                        .justify_end()
                                        .w_full()
                                        .gap(px(2.))
                                        .child(
                                            Button::new("cancel_edit_message")
                                                .ghost()
                                                .compact()
                                                .icon(
                                                    Icon::new(IconName::WindowClose)
                                                        .text_color(cx.theme().danger),
                                                )
                                                .tooltip("Cancel")
                                                .when_some(
                                                    self.on_cancel_click,
                                                    |this, on_cancel_click| {
                                                        this.on_click(cx, on_cancel_click)
                                                    },
                                                ),
                                        )
                                        .child(
                                            Button::new("submit_edit_message")
                                                .icon(Icon::new(IconName::CornerDownLeft))
                                                .compact()
                                                .ghost()
                                                .when_some(
                                                    self.on_submit_click,
                                                    |this, on_submit_click| {
                                                        this.on_click(cx, on_submit_click)
                                                    },
                                                ),
                                        ),
                                )
                            }),
                    )
                    .child(
                        // Action buttons row
                        h_flex()
                            .justify_end()
                            .gap(px(3.))
                            .invisible()
                            .text_color(cx.theme().muted_foreground)
                            .text_sm()
                            .group_hover("message", |t| t.visible())
                            .when(self.focused, |t| t.visible().child("c to"))
                            .child(
                                Button::new("copy_message")
                                    .small()
                                    .ghost()
                                    .icon(IconName::Copy)
                                    .when_some(self.on_copy_click, |this, on_copy_click| {
                                        this.on_click(cx, on_copy_click)
                                    }),
                            )
                            .when(self.focused, |t| t.visible().child("e to"))
                            .child(
                                Button::new("edit_message")
                                    .small()
                                    .ghost()
                                    .icon(IconName::Pencil)
                                    .when_some(self.on_edit_click, |this, on_edit_click| {
                                        this.on_click(cx, on_edit_click)
                                    })
                                    .not_focusable(),
                            ),
                    ),
            )
    }
}
pub struct ActiveChat {
    pub chat_state: Entity<ChatState>,
    pub edit_message_textarea: Entity<InputState>,
    pub chat_textarea: Entity<InputState>,
    pub focus_handle: FocusHandle,
    pub list_state: ListState,
}

pub enum ActiveChatEvent {
    Focused,
    Blurred,
}

impl EventEmitter<ActiveChatEvent> for ActiveChat {}

impl ActiveChat {
    fn submit_message(&self, cx: &mut Context<Self>) {
        self.chat_state.update(cx, |state, cx| {
            let text = self.chat_textarea.read(cx).value();
            if text.trim().is_empty() {
                return;
            }
            let id = state.add_message(Role::User, vec![Part::Text(text.trim().into())]);
            self.list_state.splice(id..id, 1);
            self.list_state.reset(state.messages.len());
        });
        let window_handle = cx.active_window();

        if let Some(window) = window_handle {
            window
                .update(cx, |_, window, cx| {
                    self.chat_textarea
                        .update(cx, |s, cx| s.set_value("", window, cx));
                })
                .ok();
        }
        cx.notify();
    }

    fn start_editing_message(
        &mut self,
        id: usize,
        text_part: SharedString,
        window: Option<&mut Window>,
        cx: &mut Context<Self>,
    ) {
        self.chat_state.update(cx, |state, _cx| {
            state.edit_message_id = Some(id);
        });

        focus::enable_focus_handles(
            cx,
            vec![
                "edit_message_textarea".into(),
                "cancel_edit_message".into(),
                "submit_edit_message".into(),
            ],
        );

        let textarea = self.edit_message_textarea.clone();
        let focus_handle = self.edit_message_textarea.focus_handle(cx);

        let apply_changes = |window: &mut Window, cx: &mut App| {
            textarea.update(cx, |state, cx_state| {
                state.set_value(text_part.clone(), window, cx_state);
                state.move_cursor_to_end(window, cx_state);
            });
            window.focus(&focus_handle);
        };

        if let Some(window) = window {
            apply_changes(window, cx);
        } else if let Some(window_handle) = cx.active_window() {
            window_handle
                .update(cx, |_, window, cx| {
                    apply_changes(window, cx);
                })
                .ok();
        }

        cx.notify();
    }

    fn on_cancel_click(&mut self, window: Option<&mut Window>, cx: &mut Context<Self>) {
        self.chat_state.update(cx, |state, cx| {
            state.edit_message_id = None;
            let apply_changes = |window: &mut Window, cx: &mut App| {
                if state.focused_message_idx.is_some() {
                    window.focus(&self.focus_handle);
                } else {
                    window.focus(&self.chat_textarea.focus_handle(cx));
                }
            };
            if state.focused_message_idx.is_some() {
                if let Some(window) = window {
                    apply_changes(window, cx);
                } else if let Some(window_handle) = cx.active_window() {
                    window_handle
                        .update(cx, |_, window, cx| {
                            apply_changes(window, cx);
                        })
                        .ok();
                }
            }
        });
        focus::disable_focus_handles(
            cx,
            vec![
                "edit_message_textarea".into(),
                "cancel_edit_message".into(),
                "submit_edit_message".into(),
            ],
        );
        cx.notify();
    }

    fn submit_edit_message(&mut self, window: Option<&mut Window>, cx: &mut Context<Self>) {
        let id = self.chat_state.read(cx).edit_message_id;
        if let Some(id) = id {
            let new_text = self
                .edit_message_textarea
                .read(cx)
                .value()
                .trim()
                .to_string();
            if new_text.is_empty() {
                return;
            }

            self.chat_state.update(cx, |state, _cx| {
                state.edit_message(id, vec![Part::Text(new_text)]);
            });
            self.on_cancel_click(window, cx);
            cx.notify();
        }
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let a = Self {
            chat_state: cx.new(|cx| ChatState::new(window, cx)),
            edit_message_textarea: cx
                .new(|cx| InputState::new(window, cx).multi_line().auto_grow(2, 6)),
            chat_textarea: cx.new(|cx| {
                InputState::new(window, cx)
                    .multi_line()
                    .auto_grow(3, 6)
                    .placeholder("Message control")
            }),
            list_state: ListState::new(0, gpui::ListAlignment::Bottom, px(3000.), {
                let this = cx.entity().downgrade();
                move |i, window, cx| {
                    this.update(cx, |this, cx| this.render_message(i, window, cx))
                        .unwrap()
                }
            }),
            focus_handle: cx.focus_handle(),
        };

        focus::disable_focus_handles(
            cx,
            vec![
                "edit_message_textarea".into(),
                "cancel_edit_message".into(),
                "submit_edit_message".into(),
            ],
        );
        cx.subscribe(&a.chat_textarea, |this, i, e: &InputEvent, cx| match e {
            InputEvent::PressEnter { secondary: true } => this.submit_message(cx),
            InputEvent::EmptyTextUp => {
                let (id, text_part) = this
                    .chat_state
                    .read(cx)
                    .messages
                    .iter()
                    .rfind(|m| m.role == Role::User)
                    .and_then(|m| {
                        let text = m.parts.iter().find_map(|p| match p {
                            Part::Text(content) => Some(content.clone()),
                            _ => None,
                        });
                        text.map(|t| (m.id, t))
                    })
                    .unzip();

                if let (Some(id), Some(text_part)) = (id, text_part) {
                    this.start_editing_message(id, text_part.into(), None, cx);
                }
            }
            _ => {}
        })
        .detach();

        state::init(cx);

        cx.subscribe(
            &a.edit_message_textarea,
            |this, i, e: &InputEvent, cx| match e {
                InputEvent::PressEnter { secondary: true } => {
                    this.submit_edit_message(None, cx);
                }
                InputEvent::PressEscape => {
                    this.on_cancel_click(None, cx);
                }
                _ => {}
            },
        )
        .detach();

        cx.on_focus(&a.focus_handle, window, |this, _window, cx| {
            this.chat_state.update(cx, |this, cx| {
                if this.focused_message_idx.is_none() {
                    this.fake_focused_textarea = true;
                }
            });

            // we gotta do this because for some reason cx.notify doesn't work here
            cx.emit(ActiveChatEvent::Focused);
        })
        .detach();

        cx.on_blur(&a.focus_handle, window, |this, window, cx| {
            if window.is_window_active() {
                if this.chat_state.read(cx).edit_message_id.is_none() {
                    this.chat_state.update(cx, |state, _cx| {
                        state.focused_message_idx = None;
                        state.fake_focused_textarea = false;
                    });
                    cx.emit(ActiveChatEvent::Blurred);
                }
            }
        })
        .detach();
        let chat_textarea_focus_handle = a.chat_textarea.focus_handle(cx);
        cx.on_focus(&chat_textarea_focus_handle, window, |this, _window, cx| {
            this.chat_state.update(cx, |this, cx| {
                this.focused_message_idx = None;
                this.fake_focused_textarea = false;
            });
            cx.notify();
        })
        .detach();

        cx.subscribe_self(|this, event, cx| {
            cx.notify();
        })
        .detach();
        a
    }

    fn render_user_message(
        &mut self,
        msg: Message,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let id = msg.id;

        let text_part: SharedString = msg
            .parts
            .iter()
            .find_map(|p| match p {
                Part::Text(content) => Some(content.into()),
                _ => None,
            })
            .unwrap();

        let textarea: Option<AnyElement> = self
            .chat_state
            .read(cx)
            .edit_message_id
            .filter(|&eid| eid == id)
            .map(|_| {
                TextInput::new(&self.edit_message_textarea)
                    .bordered(false)
                    .appearance(false)
                    .into_any_element()
            });

        let text_part_for_copy = text_part.clone();
        let text_part_for_edit = text_part;

        let focused = self
            .chat_state
            .read(cx)
            .focused_message_idx
            .clone()
            .map(|idx| idx == msg.id)
            .unwrap_or_default();

        MessageBubble::new(msg)
            .on_copy_click(cx.listener(move |_, _, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(text_part_for_copy.to_string()));
                window.push_notification(Notification::info("Copied to Clipboard."), cx);
            }))
            .on_edit_click(cx.listener(move |this, _e, window, cx| {
                this.start_editing_message(id, text_part_for_edit.clone(), Some(window), cx);
            }))
            .on_cancel_click(cx.listener(|this, _e, window, cx| {
                this.on_cancel_click(Some(window), cx);
                cx.notify();
            }))
            .on_submit_click(cx.listener(|this, _e, window, cx| {
                this.submit_edit_message(Some(window), cx);
                cx.notify();
            }))
            .edit_message_view(textarea)
            .focused(focused)
    }

    fn render_assistant_message(
        &mut self,
        _msg: &Message,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl gpui::IntoElement {
        div()
    }

    pub fn render_message(
        &mut self,
        msg: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let msg = self.chat_state.update(cx, |s, _cx| s.messages[msg].clone());
        if msg.role == Role::User {
            self.render_user_message(msg, window, cx).into_any_element()
        } else {
            self.render_assistant_message(&msg, window, cx)
                .into_any_element()
        }
    }
}

impl Render for ActiveChat {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let textinput = TextInput::new(&self.chat_textarea).bordered(false);

        let bg = if textinput.state.read(cx).is_disabled() {
            cx.theme().muted
        } else {
            cx.theme().background
        };
        let appearance = textinput.appearance;
        let listener = cx.listener(|_this, _, _window, cx| {
            cx.spawn(async |this: WeakEntity<Self>, cx| {
                this.update(cx, |this, cx| {
                    this.submit_message(cx);
                })
                .ok();
            })
            .detach();
        });
        let chat_state = self.chat_state.downgrade();

        println!("{}", self.list_state.viewport_bounds().size.width);

        div()
            .id("chat_route")
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .bg(cx.theme().background)
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .relative()
            .child(
                div()
                    .size_full()
                    .relative()
                    .max_w(rems(63.))
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .py(px(10.))
                    .pr(px(10.))
                    .child(
                        list(self.list_state.clone())
                            .flex_grow()
                            .size_full()
                            .relative()
                            .pb(px(30.)),
                    )
                    .h_full()
                    .child(
                        div()
                            .w_full()
                            .child(textinput)
                            .pb(px(10.))
                            .rounded(cx.theme().radius * 1.5)
                            .when(appearance, |this| {
                                this.bg(bg)
                                    .border_color(cx.theme().input)
                                    .border_1()
                                    .when(cx.theme().shadow, |this| this.shadow_sm())
                                    .when(
                                        self.chat_textarea.focus_handle(cx).is_focused(window),
                                        |this| this.focused_border(cx),
                                    )
                            })
                            .when(self.chat_state.read(cx).fake_focused_textarea, |this| {
                                this.border_2().border_color(cx.theme().ring)
                            })
                            .child(
                                div()
                                    .flex()
                                    .rounded_b(cx.theme().radius)
                                    .items_center()
                                    .justify_end()
                                    .w_full()
                                    .gap_2()
                                    .px(px(10.))
                                    .child(
                                        Button::new("submit")
                                            .primary()
                                            .disabled(
                                                self.chat_state.read(cx).streaming
                                                    || self
                                                        .chat_textarea
                                                        .read(cx)
                                                        .value()
                                                        .is_empty(),
                                            )
                                            .icon(
                                                Icon::default()
                                                    .path(IconName::ArrowUp.path())
                                                    .p(px(5.)),
                                            )
                                            .on_click(cx, listener)
                                            .not_focusable(),
                                    ),
                            ),
                    ),
            )
            .on_action({
                let chat_state = chat_state.clone();
                move |_: &UpMessage, window, cx| {
                    chat_state.update(cx, |this, cx| this.up(window, cx)).ok();
                }
            })
            .on_action({
                let chat_state = chat_state.clone();
                move |_: &DownMessage, window, cx| {
                    chat_state.update(cx, |this, cx| this.down(window, cx)).ok();
                }
            })
            .when(
                self.chat_state.read(cx).focused_message_idx.is_some()
                    && !self
                        .edit_message_textarea
                        .focus_handle(cx)
                        .is_focused(window),
                |this| {
                    this.on_action({
                        let this = cx.entity().downgrade();
                        move |_: &state::Edit, window, cx| {
                            this.update(cx, |this, cx| {
                                if let Some(id) = this.chat_state.read(cx).focused_message_idx {
                                    let msg = this
                                        .chat_state
                                        .read(cx)
                                        .messages
                                        .iter()
                                        .find(|m| m.id == id)
                                        .cloned();

                                    if let Some(msg) = msg {
                                        if msg.role == Role::User {
                                            let text_part = msg
                                                .parts
                                                .iter()
                                                .find_map(|p| match p {
                                                    Part::Text(content) => {
                                                        Some(SharedString::from(content))
                                                    }
                                                    _ => None,
                                                })
                                                .unwrap_or_default();
                                            this.start_editing_message(
                                                id,
                                                text_part,
                                                Some(window),
                                                cx,
                                            );
                                        }
                                    }
                                }
                            })
                            .ok();
                        }
                    })
                    .on_action({
                        let chat_state = chat_state.clone();
                        move |_: &state::Copy, window, cx| {
                            if let Some(chat_state) = chat_state.upgrade() {
                                let state = chat_state.read(cx);
                                if let Some(id) = state.focused_message_idx {
                                    if let Some(msg) = state.messages.iter().find(|m| m.id == id) {
                                        let text_part: String = msg
                                            .parts
                                            .iter()
                                            .find_map(|p| match p {
                                                Part::Text(content) => Some(content.clone()),
                                                _ => None,
                                            })
                                            .unwrap_or_default();

                                        if !text_part.is_empty() {
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                text_part,
                                            ));
                                            window.push_notification(
                                                Notification::info("Copied to Clipboard."),
                                                cx,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    })
                },
            )
            .when(self.chat_state.read(cx).fake_focused_textarea, |this| {
                this.on_key_down({
                    let this = cx.entity().downgrade();
                    move |ev, window, cx| {
                        if ev.keystroke.key == "tab" {
                            return;
                        }
                        this.update(cx, |this, cx| {
                            this.chat_state.update(cx, |this, cx| {
                                this.fake_focused_textarea = false;
                                this.focused_message_idx = None;
                            });
                            cx.notify();
                        })
                        .ok();

                        window.focus(&focus::get_or_create_focus_handle(
                            cx,
                            "chat_textarea".into(),
                        ));

                        window.dispatch_keystroke(ev.keystroke.clone(), cx);
                        cx.stop_propagation();
                    }
                })
            })
        // .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
        //     if event.keystroke.key == "tab".to_string() {
        //         if event.keystroke.modifiers.shift {
        //             focus::focus_previous(window, cx);
        //         } else {
        //             focus::focus_next(window, cx);
        //         }
        //     } else if event.keystroke.key == "enter" {
        //         focus::handle_enter_focus_event_with_window(window, cx);
        //     }
        //     window.focus(&this.chat_textarea.focus_handle(cx));
        //     this.chat_state.update(cx, |this, cx| {
        //         this.focused_mode = false;
        //         this.fake_focused_textarea = false;
        //         this.focused_message_idx = None;
        //     })
        // }))
    }
}
