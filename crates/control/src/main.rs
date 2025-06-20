mod config;

use gpui::{
    div, prelude::*, px, rems, size, transparent_black, AnyElement, AnyView, App, Application, Bounds, ClickEvent, Context, Decorations, ElementId, Entity, EventEmitter, Focusable, Global, SharedString, Window, WindowBounds, WindowDecorations, WindowOptions
};
use rfd::FileDialog;
use ui::{
    theme,
    focus::{self, EnterFocusEvent}, h_flex, highlighter, input::{self, InputEvent, InputState, TextInput}, notification::Notification, theme::{ActiveTheme, Theme, ThemeMode}, v_flex, Assets, Button, ButtonVariants, ContextModal, Icon, IconName, NewTaskSidebar, Root, Sidebar, SidebarToggleButton, StyledExt, TitleBar
};
use crate::config::{AppConfig, load_config, save_config};
use crate::config::ActiveConfig;

#[derive(Debug, Clone, Copy)]
enum Route {
    Home,
    Chat,
    Settings
}


impl Route {
    fn cycle(&self) -> Vec<impl Into<ElementId>> {
        let mut titlebar = vec!["collapse", "theme-selector", "minimize", "zoom", "close"];

        let r =match self {
            Self::Home => vec!["new-task", "settings", "textarea_main", "working_dir", "submit"],
            Self::Settings => Vec::new(),
            Self::Chat => Vec::new(),
        };

        titlebar.extend(&r);

        titlebar
    }
}
impl Global for Route {}


trait Navigation {
    fn goto(&mut self, route: Route);
}

impl <'a, T: 'static> Navigation for Context<'a, T> {
    fn goto(&mut self, route: Route) {
        focus::set_focus_cycle(self, route.cycle());
        self.set_global(route);
        self.notify();
    }
}

// --- Control Root Structure ---
struct ControlRoot {
    title_bar: Entity<ControlTitleBar>,
    view: AnyView,
    sidebar_collapsed: bool,
}

impl ControlRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| ControlTitleBar::new(title, cx));

        cx.subscribe(&title_bar, |this, _titlebar, event, cx| {
            if *event == TitleBarEvent::ToggleCollapse {
                this.sidebar_collapsed = !this.sidebar_collapsed;
                cx.notify();
            }
        }).detach();

     
        Self {
            title_bar,
            view: view.into(),
            sidebar_collapsed: false,
        }
    }
}

impl Render for ControlRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rounded_size = cx.theme().radius;

        let settings_listener=
            cx.listener(|_this, _: &ClickEvent, _window, cx| {
                cx.goto(Route::Settings);
            });

        let settings_button =
            Button::new("settings").w_full()
                .outline()
                .when(!self.sidebar_collapsed, |this | this.label("Settings"))
                .icon(IconName::Settings)
                .on_click(cx, settings_listener);

        
        let notification_layer = Root::render_notification_layer(window, cx);
        v_flex()
            .size_full()
            .rounded(rounded_size)
            .child(self.title_bar.clone())
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden() 
                    .child(
                        div()
                            .p(px(10.))
                            .h_full() 
                            .child(
                                Sidebar::left()
                                    .collapsible(true)
                                    .collapsed(self.sidebar_collapsed)
                                    .floating(true)
                                    .width(px(230.))
                                    
                                    .child(NewTaskSidebar::new().on_new_task(|_ev, window, cx| {
                                        window.push_notification(Notification::info("Creating new task..."), cx);
                                    }))
                                    .footer(
                                        settings_button
                                    )

                        )
                    )
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.view.clone())
                    )
            )
            .child(div().absolute().top_12().children(notification_layer))
    }
}

// --- Control Title Bar ---
struct ControlTitleBar {
    title: SharedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TitleBarEvent {
    ToggleCollapse
}

impl ControlTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl EventEmitter<TitleBarEvent> for ControlTitleBar {}
impl Render for ControlTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rounded_size = px(cx.config().ui_settings.rounded_size);

        TitleBar::new()
            .child(
                h_flex()
                    .id("title-text")
                    .gap(px(2.))
                    .font_semibold()
                    .child(SidebarToggleButton::left().on_click(cx.listener(|_this, _event, _window, cx| {
                        cx.emit(TitleBarEvent::ToggleCollapse);
                    })))
                    .on_click(cx.listener(|_this, _event, _window, cx| cx.goto(Route::Home)))
                    .child(self.title.clone())
                    .text_color(cx.theme().foreground)
            )
            .child(
                Button::new("theme-selector").icon(IconName::Sun).on_click(cx, |_, window, cx| {
                    let new_mode = match cx.theme().mode {
                        ThemeMode::Light => ThemeMode::Dark,
                        ThemeMode::Dark => ThemeMode::Light,
                    };
                    Theme::change(new_mode, Some(window), cx);
                    
                    let mut new_config = cx.config().clone();
                    new_config.theme_mode = new_mode;
                    save_config(&new_config).ok();
                    *cx.global_mut::<AppConfig>() = new_config;
                }).outline()
            )
            .border_b_1()
            .border_color(cx.theme().border)
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                el.rounded_tl(rounded_size)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                el.rounded_tr(rounded_size)
            })
    }
}

// --- Main Application View ---
pub struct MainApp {
    textarea: Entity<InputState>,
}

impl MainApp {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let m = MainApp {
            textarea: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Describe your task")
                    .multi_line()
                    .auto_grow(3, 6)
            }),
        };

        focus::register_focusable(cx, "textarea_main".into(), m.textarea.focus_handle(cx));
        m.textarea.focus_handle(cx).focus(window);

        cx.subscribe(&m.textarea, |_i, e: &InputEvent, _cx| {
            if matches!(e, InputEvent::PressEnter { secondary: true }) {
                dbg!("Submit triggered");
            }
        }).detach();

        cx.set_global(Route::Home);
        focus::set_focus_cycle(cx, Route::Home.cycle());
        m
    } 

    fn render_settings_route(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("settings_route")
            .size_full()
            .bg(cx.theme().background)
            .flex()
            .flex_col()
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                el.rounded_bl(cx.theme().radius)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                el.rounded_br(cx.theme().radius)
            })
            .child(div().child("Settings!"))
    }

    fn render_home_route(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let textinput = TextInput::new(&self.textarea).bordered(false);
        let bg = if textinput.state.read(cx).is_disabled() {
            cx.theme().muted
        } else {
            cx.theme().background
        };
        let appearance = textinput.appearance;
        let on_file_click = cx.listener(|_this, _event: &ClickEvent, window, cx| {
            if let Some(p) = FileDialog::new().pick_folder() {
                if p.to_str().unwrap_or("") != "" {
                    let mut new_config = cx.config().clone();
                    new_config.working_dir = Some(p);
                    save_config(&new_config).ok();
                    *cx.global_mut::<AppConfig>() = new_config;
                }
            } else {
                window.push_notification(Notification::error("Please select a folder."), cx);
            }
        });
        div()
            .id("home_route")
            .size_full()
            .bg(cx.theme().background)
            .flex()
            .flex_col()
            .gap_4()
            .justify_center()
            .items_center()
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                el.rounded_bl(cx.theme().radius)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                el.rounded_br(cx.theme().radius)
            })
            .p_8()
            .child(
                div().child("What can I help you with?").font_semibold().text_size(px(32.)).text_center()
            )
            .child(
                div()
                    .w_full() 
                    .max_w(rems(48.)) 
                    .child(
                        textinput
                    )
                    .pb(px(10.))
                    .rounded(cx.theme().radius)
                    .when(appearance, |this| {
                        this.bg(bg)
                          .border_color(cx.theme().input)
                          .border_1()
                          .when(cx.theme().shadow, |this| this.shadow_sm())
                          .when(self.textarea.read(cx).focus_handle(cx).is_focused(window), |this| this.focused_border(cx))
                    })
                
                    .child(
                        div()
                            .flex()
                            .rounded_b(cx.theme().radius)
                            .items_center()
                            .justify_between()
                            .w_full() 
                            .gap_2()
                            .px(px(10.))
                            .child(
                                Button::new("working_dir").outline().icon(IconName::Folder).compact()
                                    .label(cx.config().working_dir.as_ref().map(|p| p.as_os_str().to_str().unwrap_or("...").to_string()).unwrap_or_else(|| "Unselected".into()))
                                    .on_click(cx, on_file_click),
                            )
                            .child(
                                Button::new("submit").primary().icon(Icon::default().path(IconName::ArrowUp.path()).p(px(5.))).on_click(cx, |_event, _window, _cx| {})
                            )
                    )
            )
    }
    
    fn render_main_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match cx.global() {
            Route::Home => self.render_home_route(window, cx).into_any_element(),
            Route::Settings => self.render_settings_route(window, cx).into_any_element(),
            _ => unreachable!()
        }
    }
    
}

impl EventEmitter<EnterFocusEvent> for MainApp {}

impl Render for MainApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_main_content(window, cx)
    }
}

// --- Application Entry Point ---
fn main() {
    let config = load_config().unwrap_or_default();
    let light_colors = config.ui_settings.light;
    let dark_colors = config.ui_settings.dark;

    let t = Theme {
        all_colors: ui::theme::ThemeColorWithMode { light: light_colors.clone(), dark: dark_colors.clone() },
        colors: if config.theme_mode == ThemeMode::Dark { dark_colors } else { light_colors },
        radius: px(config.ui_settings.rounded_size),
        shadow: false,
        font_family: "Geist".into(),
        font_size: px(15.),
        tile_grid_size: px(4.),
        tile_shadow: false,
        transparent: transparent_black(),
        mode: config.theme_mode,
        scrollbar_show: ui::scroll::ScrollbarShow::Scrolling,
    };

    Application::new()
        .with_assets(Assets)
        .run(move |cx: &mut crate::App| {
            // Register AppConfig as a global
            cx.set_global::<AppConfig>(config.clone());

            cx.observe_keystrokes(|event, window, app| {
                if event.keystroke.key == "tab".to_string() {
                    if event.keystroke.modifiers.shift {
                        focus::focus_previous(window, app);
                    } else {
                        focus::focus_next(window, app);
                    }
                } else if event.keystroke.key == "enter" {
                    focus::handle_enter_focus_event_with_window(window, app);
                }
            })
            .detach();
            let initial_size = size(px(1024.), px(500.));
            let bounds = Bounds::centered(None, initial_size, cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    is_movable: true,
                    kind: gpui::WindowKind::Normal,
                    window_background: gpui::WindowBackgroundAppearance::Transparent,
                    focus: true,
                    titlebar: Some(TitleBar::title_bar_options()),
                    app_id: Some("Control".into()),
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |window, cx| {
                    theme::init(cx, &t);
                    highlighter::init(cx);
                    input::init(cx);
                    Theme::change(config.theme_mode, None, cx);
                    println!("{:?}", window.gpu_specs());
                    focus::init(cx);
                    let main_app = cx.new(|cx| MainApp::new(window, cx));
                    let control_root =
                        cx.new(|cx| ControlRoot::new("Control", main_app.clone(), window, cx));
                    cx.new(|cx| Root::new(control_root.into(), window, cx))
                },
            )
            .unwrap();

            cx.activate(true);
        });
}
