use gpui::{
    actions, div, hsla, prelude::*, px, rems, rgb, size, transparent_black, App, Application, Bounds, Context, Decorations, Entity, EventEmitter, KeyBinding, MouseButton, Pixels, SharedString, Window, WindowBounds, WindowDecorations, WindowOptions
};
use ui::{
    colors::{self, Colorize}, focus::{self, EnterFocusEvent}, theme::{self, hsl, ActiveTheme, Theme, ThemeColor, ThemeMode}, Button, ButtonVariants
};

actions!(main_app, [FocusNext]);

const ROUNDED_SIZE: Pixels = px(15.);

struct Titlebar;

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Basic styling inspired by Zed's title bar
        let title_bar_height = rems(3.); // Approximate height
        let title_bar_bg = cx.theme().background; // Dark background
        let title_bar_text_color = cx.theme().foreground; // Light text
        
        div()
            .id("simple-titlebar")
            .w_full() // Take full width
            .h(title_bar_height) // Set fixed height
            .bg(title_bar_bg) // Set background color
            // Apply top rounding if the window isn't tiled at the top corners
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.left)), |el| {
                el.rounded_tl(ROUNDED_SIZE)
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.right)), |el| {
                el.rounded_tr(ROUNDED_SIZE)
            })
            .flex() // Use flexbox for internal layout
            .items_center() // Center items vertically
            .pl_2() // Add left padding
            .child(
                div()
                    .id("hey")
                    .child("Control")
                    .text_color(title_bar_text_color)
                    .on_mouse_up(MouseButton::Left, |_, win, cx| {
                        println!("meow");


                        match cx.theme().is_dark() {
                            true => Theme::change(ThemeMode::Light, Some(win), cx),
                            false => Theme::change(ThemeMode::Dark, Some(win), cx),
                        }
                    })
            )
            // Make the title bar draggable

            .on_mouse_down(gpui::MouseButton::Left, |_, win, _| {
                win.start_window_move(); // Allows dragging the window by the title bar
            })
    }
}


// --- Main Application View ---
pub struct MainApp {}

impl EventEmitter<EnterFocusEvent> for MainApp {}

impl Render for MainApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Main container using vertical flex to stack title bar and content
        div()
            .id("main_app")
            .flex()
            .flex_col()
            .size_full() // Fill the entire window
            .child(cx.new(|_| Titlebar {}))

            .child(
                // Original content area
                div()
                    .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.left)), |el| {
                        el.rounded_bl(ROUNDED_SIZE)
                    })
                    .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.bottom || tiling.right)), |el| {
                        el.rounded_br(ROUNDED_SIZE)
                    })
                    .bg(cx.theme().background) // Background for the content area
                    .flex_grow() // Allow this area to fill the remaining vertical space
                    .w_full() // Take full width
                    .flex() // Use flex to center the text
                    .flex_col()
                    .gap_2()
                    .justify_center()
                    .items_center()
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        // Unfocus all elements when clicking on empty space
                        focus::unfocus_all(window);
                    })
                    .child(
                        Button::new("button1")
                            .label("Solid")
                            .primary()
                            .on_click_with_enter(cx, |_, _, _| {
                                println!("Solid button clicked!");
                            })
                    )
                    .child(
                        Button::new("button2")
                            .label("Outline")
                            .outline()
                            .on_click_with_enter(cx, |_, _, _| {
                                println!("Outline button clicked!");
                            })
                    )
                    .child(
                        Button::new("button3")
                            .label("Destructive")
                            .danger()
                            .on_click_with_enter(cx, |_, _, _| {
                                println!("Destructive button clicked!");
                            })
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
        ring: hsl(240.0, 4.9, 83.9),
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
        font_family: "Cantrell".into(),
        font_size: px(15.),
        tile_grid_size: px(4.),
        tile_shadow: false,
        transparent: transparent_black(),
        mode: ThemeMode::Dark,
        scrollbar_show: ui::scroll::ScrollbarShow::Scrolling,
    };

    Application::new().run(move |cx: &mut crate::App| {
        cx.bind_keys(vec![KeyBinding::new("tab", FocusNext, None)]);
        cx.observe_keystrokes(|event, window, app| {
            if event.keystroke.key == "tab".to_string() {
                if event.keystroke.modifiers.shift {
                    println!("Shift+Tab pressed - focusing previous");
                    focus::focus_previous(window, app);
                } else {
                    println!("Tab pressed - focusing next");
                    focus::focus_next(window, app);
                }
            }
            else if event.keystroke.key == "enter" {
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
                titlebar: None,

                app_id: Some("Control".into()),
                window_decorations: Some(WindowDecorations::Client), // IMPORTANT: Enable client-side decorations
                // -------------------------------------
                ..Default::default()
            },
            |window, cx| {
                theme::init(cx, &t);
                Theme::change(ThemeMode::Dark, None, cx);
                let entity = cx.new(|_| MainApp {});
                focus::init(cx, entity.clone().into());
                entity
            },
        )
        .unwrap();

        cx.activate(true); // Activate the application
    });
}
