use gpui::{
    div, hsla, prelude::*, px, size, svg, transparent_black, AnyView, App, Application, Bounds, Context, Decorations, Entity, EventEmitter, Focusable, Font, MouseButton, Pixels, SharedString, Window, WindowBounds, WindowDecorations, WindowOptions
};
use native_dialog::DialogBuilder;
use ui::{
    colors::{self, Colorize}, focus::{self, EnterFocusEvent}, highlighter, input::{self, InputState, TextInput}, notification::Notification, theme::{self, hsl, ActiveTheme, Theme, ThemeColor, ThemeMode}, v_flex, Assets, Button, ButtonVariants, ContextModal, Icon, IconName, Root, Sizable, Size, StyledExt, TitleBar
};

const ROUNDED_SIZE: Pixels = px(15.);

// --- Control Root Structure ---
struct ControlRoot {
    title_bar: Entity<ControlTitleBar>,
    view: AnyView,
}

impl ControlRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| ControlTitleBar::new(title, window, cx));
        Self {
            title_bar,
            view: view.into(),
        }
    }
}

impl Render for ControlRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        let notification_layer = Root::render_notification_layer(window, cx);

        
        v_flex()
            .size_full()
            .rounded(ROUNDED_SIZE)
            // .border_1()
            // .border_color(cx.theme().border)
            .child(self.title_bar.clone())
            .child(div().flex_1().overflow_hidden().child(self.view.clone()))
            .child(div().absolute().top_12().children(notification_layer))
    }
}

// --- Control Title Bar ---
struct ControlTitleBar {
    title: SharedString,
}

impl ControlTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Render for ControlTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(
                div()
                    .id("title-text")
                    .child(self.title.clone())
                    .text_color(cx.theme().foreground)
            )
            .child(
                Button::new("theme-selector").icon(IconName::Sun).on_click_with_index(cx, 0, |_, window, cx| {
                    match cx.theme().mode {
                        ThemeMode::Light => Theme::change(ThemeMode::Dark, Some(window), cx),
                        ThemeMode::Dark => Theme::change(ThemeMode::Light, Some(window), cx),
                    }
                }).outline()
            )
            .border_b_1()
            .border_color(cx.theme().border)
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                el.rounded_tl(ROUNDED_SIZE)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                el.rounded_tr(ROUNDED_SIZE)
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
        m
    }
}

impl EventEmitter<EnterFocusEvent> for MainApp {}

impl Render for MainApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Content area without title bar (title bar will be handled by ControlRoot)
        let textinput = TextInput::new(&self.textarea).bordered(false);
        let bg = if textinput.state.read(cx).is_disabled() {
            cx.theme().muted
        } else {
            cx.theme().background
        };
        let appearance = textinput.appearance;
        div()
            .id("main_app")
            .size_full()
            .bg(cx.theme().background)
            .flex()
            .flex_col()
            .gap_4()
            .justify_center()
            .items_center()
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                el.rounded_bl(ROUNDED_SIZE)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                el.rounded_br(ROUNDED_SIZE)
            })
            .p_8()
            .child(
                div().child("What can I help you with?").font_semibold().text_size(px(32.)).text_center()
            )
            .child(
                div()
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
                            .size_full()
                            .gap_2()
                            .px(px(10.))
                            .child(
                                Button::new("working_dir").outline().icon(IconName::Folder).compact().label("/home/oubra").on_click(cx, |_, window, cx| {
                                    match DialogBuilder::file().open_single_dir().show() {
                                        Err(x) => window.push_notification(Notification::error(format!("Failed to open folder dialog: {x}")), cx),
                                        Ok(dir) => match dir {
                                            None => {
                                                window.push_notification(Notification::error("Please select a folder."), cx)
                                            },
                                            Some(_) => {}
                                        }
                                    };
                                }) 
                            )
                            .child(
                                Button::new("submit").primary().icon(Icon::default().path(IconName::ArrowUp.path()).p(px(5.)))
                            )
                )
        )
    }
}

// --- Application Entry Point ---
fn main() {
    let light = ThemeColor {
        accent: hsl(240.0, 4.8, 93.9),
        accent_foreground: hsl(240.0, 5.9, 10.0),
        accordion: hsl(0.0, 0.0, 100.0),
        accordion_active: hsl(240.0, 5.9, 90.0),
        accordion_hover: hsl(240.0, 4.8, 95.9).opacity(0.7),
        background: hsl(0.0, 0.0, 100.),
        border: hsl(240.0, 5.9, 90.0),
        card: hsl(0.0, 0.0, 100.0),
        card_foreground: hsl(240.0, 10.0, 3.9),
        caret: hsl(240.0, 10., 3.9),
        danger: colors::red_500(),
        danger_active: colors::red_600(),
        danger_foreground: colors::red_50(),
        danger_hover: colors::red_500().opacity(0.9),
        description_list_label: hsl(240.0, 5.9, 96.9),
        description_list_label_foreground: hsl(240.0, 5.9, 10.0),
        drag_border: colors::blue_500(),
        drop_target: hsl(235.0, 30., 44.0).opacity(0.25),
        foreground: hsl(240.0, 10., 3.9),
        info: colors::sky_500(),
        info_active: colors::sky_600(),
        info_hover: colors::sky_500().opacity(0.9),
        info_foreground: colors::sky_50(),
        input: hsl(240.0, 5.9, 90.0),
        link: hsl(221.0, 83.0, 53.0),
        link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
        link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
        list: hsl(0.0, 0.0, 100.),
        list_active: hsl(211.0, 97.0, 85.0).opacity(0.2),
        list_active_border: hsl(211.0, 97.0, 85.0),
        list_even: hsl(240.0, 5.0, 96.0),
        list_head: hsl(0.0, 0.0, 100.),
        list_hover: hsl(240.0, 4.8, 95.0),
        muted: hsl(240.0, 4.8, 95.9),
        muted_foreground: hsl(240.0, 3.8, 46.1),
        popover: hsl(0.0, 0.0, 100.0),
        popover_foreground: hsl(240.0, 10.0, 3.9),
        primary: hsl(223.0, 5.9, 10.0),
        primary_active: hsl(223.0, 1.9, 25.0),
        primary_foreground: hsl(223.0, 0.0, 98.0),
        primary_hover: hsl(223.0, 5.9, 15.0),
        progress_bar: hsl(223.0, 5.9, 10.0),
        ring: hsl(223.81, 0., 6.),
        scrollbar: hsl(0., 0., 98.).opacity(0.95),
        scrollbar_thumb: hsl(0., 0., 69.).opacity(0.9),
        scrollbar_thumb_hover: hsl(0., 0., 59.),
        secondary: hsl(240.0, 5.9, 96.9),
        secondary_active: hsl(240.0, 5.9, 93.),
        secondary_foreground: hsl(240.0, 59.0, 10.),
        secondary_hover: hsl(240.0, 5.9, 98.),
        selection: hsl(211.0, 97.0, 85.0),
        sidebar: hsl(0.0, 0.0, 98.0),
        sidebar_accent: colors::zinc_200(),
        sidebar_accent_foreground: hsl(240.0, 5.9, 10.0),
        sidebar_border: hsl(220.0, 13.0, 91.0),
        sidebar_foreground: hsl(240.0, 5.3, 26.1),
        sidebar_primary: hsl(240.0, 5.9, 10.0),
        sidebar_primary_foreground: hsl(0.0, 0.0, 98.0),
        skeleton: hsl(223.0, 5.9, 10.0).opacity(0.1),
        slider_bar: hsl(223.0, 5.9, 10.0),
        slider_thumb: hsl(0.0, 0.0, 100.0),
        success: colors::green_500(),
        success_active: colors::green_600(),
        success_hover: colors::green_500().opacity(0.9),
        success_foreground: colors::gray_50(),
        switch: colors::zinc_300(),
        tab: gpui::transparent_black(),
        tab_active: hsl(0.0, 0.0, 100.0),
        tab_active_foreground: hsl(240.0, 10., 3.9),
        tab_bar: hsl(240.0, 14.3, 95.9),
        tab_bar_segmented: hsl(240.0, 14.3, 95.9),
        tab_foreground: hsl(240.0, 10., 33.9),
        table: hsl(0.0, 0.0, 100.),
        table_active: hsl(211.0, 97.0, 85.0).opacity(0.2),
        table_active_border: hsl(211.0, 97.0, 85.0),
        table_even: hsl(240.0, 5.0, 96.0),
        table_head: hsl(0.0, 0.0, 100.),
        table_head_foreground: hsl(240.0, 5.0, 34.),
        table_hover: hsl(240.0, 4.8, 95.0),
        table_row_border: hsl(240.0, 7.7, 94.5),
        tiles: hsl(0.0, 0.0, 95.),
        title_bar: hsl(0.0, 0.0, 100.),
        title_bar_border: hsl(240.0, 5.9, 90.0),
        warning: colors::yellow_500(),
        warning_active: colors::yellow_600(),
        warning_hover: colors::yellow_500().opacity(0.9),
        warning_foreground: colors::gray_50(),
        window_border: hsl(240.0, 5.9, 78.0),
    };
    let dark = ThemeColor {
        accent: hsl(240.0, 3.7, 15.9),
        accent_foreground: hsl(0.0, 0.0, 78.0),
        accordion: hsl(299.0, 2., 11.),
        accordion_active: hsl(240.0, 3.7, 16.9),
        accordion_hover: hsl(240.0, 3.7, 15.9).opacity(0.7),
        background: hsl(0.0, 0.0, 8.0),
        border: hsl(240.0, 3.7, 16.9),
        card: hsl(0.0, 0.0, 8.0),
        card_foreground: hsl(0.0, 0.0, 78.0),
        caret: hsl(0., 0., 78.),
        danger: colors::red_800(),
        danger_active: colors::red_800().darken(0.2),
        danger_foreground: colors::red_50(),
        danger_hover: colors::red_800().opacity(0.9),
        description_list_label: hsl(240.0, 0., 13.0),
        description_list_label_foreground: hsl(0.0, 0.0, 78.0),
        drag_border: colors::blue_500(),
        drop_target: hsl(235.0, 30., 44.0).opacity(0.1),
        foreground: hsl(0., 0., 78.),
        info: colors::sky_900(),
        info_active: colors::sky_900().darken(0.2),
        info_foreground: colors::sky_50(),
        info_hover: colors::sky_900().opacity(0.8),
        input: hsl(240.0, 3.7, 15.9),
        link: hsl(221.0, 83.0, 53.0),
        link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
        link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
        list: hsl(0.0, 0.0, 8.0),
        list_active: hsl(240.0, 3.7, 15.0).opacity(0.2),
        list_active_border: hsl(240.0, 5.9, 35.5),
        list_even: hsl(240.0, 3.7, 10.0),
        list_head: hsl(0.0, 0.0, 8.0),
        list_hover: hsl(240.0, 3.7, 15.9),
        muted: hsl(240.0, 3.7, 15.9),
        muted_foreground: hsl(240.0, 5.0, 34.),
        popover: hsl(0.0, 0.0, 10.),
        popover_foreground: hsl(0.0, 0.0, 78.0),
        primary: hsl(223.0, 0.0, 98.0),
        primary_active: hsl(223.0, 0.0, 80.0),
        primary_foreground: hsl(223.0, 5.9, 10.0),
        primary_hover: hsl(223.0, 0.0, 90.0),
        progress_bar: hsl(223.0, 0.0, 98.0),
        ring: hsl(240.0, 4.9, 40.9),
        scrollbar: hsl(240.0, 0.0, 10.0).opacity(0.95),
        scrollbar_thumb: hsl(0., 0., 48.).opacity(0.9),
        scrollbar_thumb_hover: hsl(0., 0., 68.),
        secondary: hsl(240.0, 0., 13.0),
        secondary_active: hsl(240.0, 0., 13.),
        secondary_foreground: hsl(0.0, 0.0, 78.0),
        secondary_hover: hsl(240.0, 0., 15.),
        selection: hsl(211.0, 97.0, 22.0),
        sidebar: hsl(240.0, 0.0, 10.0),
        sidebar_accent: colors::zinc_800(),
        sidebar_accent_foreground: hsl(240.0, 4.8, 95.9),
        sidebar_border: hsl(240.0, 3.7, 15.9),
        sidebar_foreground: hsl(240.0, 4.8, 95.9),
        sidebar_primary: hsl(0.0, 0.0, 98.0),
        sidebar_primary_foreground: hsl(240.0, 5.9, 10.0),
        skeleton: hsla(223.0, 0.0, 98.0, 0.1),
        slider_bar: hsl(223.0, 0.0, 98.0),
        slider_thumb: hsl(0.0, 0.0, 8.0),
        success: colors::green_800(),
        success_active: colors::green_800().darken(0.2),
        success_foreground: colors::green_50(),
        success_hover: colors::green_800().opacity(0.8),
        switch: colors::zinc_600(),
        tab: gpui::transparent_black(),
        tab_active: hsl(0.0, 0.0, 8.0),
        tab_active_foreground: hsl(0., 0., 78.),
        tab_bar: hsl(299.0, 0., 5.5),
        tab_bar_segmented: hsl(299.0, 0., 5.5),
        tab_foreground: hsl(0., 0., 78.),
        table: hsl(0.0, 0.0, 8.0),
        table_active: hsl(240.0, 3.7, 15.0).opacity(0.2),
        table_active_border: hsl(240.0, 5.9, 35.5),
        table_even: hsl(240.0, 3.7, 10.0),
        table_head: hsl(0.0, 0.0, 8.0),
        table_head_foreground: hsl(0., 0., 78.).opacity(0.7),
        table_hover: hsl(240.0, 3.7, 15.9).opacity(0.5),
        table_row_border: hsl(240.0, 3.7, 16.9).opacity(0.5),
        tiles: hsl(0.0, 0.0, 5.0),
        title_bar: hsl(0., 0., 9.7),
        title_bar_border: hsl(240.0, 3.7, 15.9),
        warning: colors::yellow_800(),
        warning_active: colors::yellow_800().darken(0.2),
        warning_foreground: colors::yellow_50(),
        warning_hover: colors::yellow_800().opacity(0.9),
        window_border: hsl(240.0, 3.7, 28.0),
    };

    let t = Theme {
        all_colors: ui::theme::ThemeColorWithMode { light, dark },
        colors: light,
        radius: px(10.),
        shadow: false,
        font_family: "Geist".into(),
        font_size: px(15.),
        tile_grid_size: px(4.),
        tile_shadow: false,
        transparent: transparent_black(),
        mode: ThemeMode::Dark,
        scrollbar_show: ui::scroll::ScrollbarShow::Scrolling,
    };

    Application::new()
        .with_assets(Assets)
        .run(move |cx: &mut crate::App| {
            cx.observe_keystrokes(|event, window, app| {
                println!("pressed {:?}", event.keystroke);
                if event.keystroke.key == "tab".to_string() {
                    if event.keystroke.modifiers.shift {
                        println!("Shift+Tab pressed - focusing previous");
                        focus::focus_previous(window, app);
                    } else {
                        println!("Tab pressed - focusing next");
                        focus::focus_next(window, app);
                    }
                } else if event.keystroke.key == "enter" {
                    // Handle Enter key press directly with window context
                    focus::handle_enter_focus_event_with_window(window, app);
                }
            })
            .detach();
            // Define initial window size and position
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
                    window_decorations: Some(WindowDecorations::Client), // IMPORTANT: Enable client-side decorations
                    // -------------------------------------
                    ..Default::default()
                },
                |window, cx| {
                    theme::init(cx, &t);
                    highlighter::init(cx);
                    input::init(cx);
                    Theme::change(ThemeMode::Dark, None, cx);

                    focus::init(cx);
                    // Create the main app view
                    let main_app = cx.new(|cx| MainApp::new(window, cx));

                    // Create the control root with title bar
                    let control_root =
                        cx.new(|cx| ControlRoot::new("Control", main_app.clone(), window, cx));

                    // Wrap everything in the Root component for modal/drawer/notification support
                    cx.new(|cx| Root::new(control_root.into(), window, cx))
                },
            )
            .unwrap();

            cx.activate(true); // Activate the application
        });
}
