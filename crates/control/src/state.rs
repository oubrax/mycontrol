use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gpui::{App, AppContext, Context, Entity, KeyBinding, Window, actions};
use ui::{focus, input::InputState};

pub const CONTEXT: &'static str = "ChatHistory";

actions!(chat_history, [UpMessage, DownMessage, Edit, Copy, Delete,]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub enum Part {
    Text(String),
    ToolCall(Tool),
}

#[derive(Debug, Clone)]
pub struct Tool {
    name: Arc<str>,
    args: Arc<[serde_json::Value]>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: usize,
    pub role: Role,
    pub parts: Vec<Part>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub messages: Vec<Message>,
    pub streaming: bool,
    pub edit_message_id: Option<usize>,
    pub focused_message_idx: Option<usize>,
    pub fake_focused_textarea: bool,
    last_id: usize,
}

impl ChatState {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            messages: Vec::new(),
            last_id: 0,
            streaming: false,
            edit_message_id: None,
            focused_message_idx: None,
            fake_focused_textarea: false,
        }
    }

    pub fn add_message(&mut self, role: Role, parts: Vec<Part>) -> usize {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();

        let timestamp = duration.as_secs();

        self.messages.push(Message {
            id: self.last_id,
            role,
            parts,
            timestamp,
        });

        self.last_id += 1;

        self.last_id - 1
    }

    pub fn edit_message(&mut self, id: usize, new_parts: Vec<Part>) {
        if let Some(message) = self.messages.iter_mut().find(|m| m.id == id) {
            message.parts = new_parts;
        }
    }

    pub fn up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            self.focused_message_idx = None;

            cx.notify();
            return;
        }

        let len = self.messages.len();
        self.focused_message_idx = self
            .focused_message_idx
            .map(|i| {
                self.fake_focused_textarea = false;
                Some(i.saturating_sub(1))
            })
            .unwrap_or_else(|| Some(len.saturating_sub(1)));

        if self.fake_focused_textarea {
            self.fake_focused_textarea = false;
        }
        cx.notify();
    }

    pub fn down(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let len = self.messages.len();
        if len == 0 {
            self.focused_message_idx = None;
            return;
        }

        self.focused_message_idx = match self.focused_message_idx {
            Some(i) if i + 1 >= len => {
                self.fake_focused_textarea = true;
                None
            }
            Some(i) => Some(i + 1),
            None => {
                if !self.fake_focused_textarea {
                    Some(0)
                } else {
                    None
                }
            }
        };

        cx.notify();
    }
}

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", UpMessage, Some(CONTEXT)),
        KeyBinding::new("down", DownMessage, Some(CONTEXT)),
        KeyBinding::new("e", Edit, Some(CONTEXT)),
        KeyBinding::new("d", Delete, Some(CONTEXT)),
        KeyBinding::new("c", Copy, Some(CONTEXT)),
    ]);
}
