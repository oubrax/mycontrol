use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Decorations, SharedString, Window, WindowBounds, WindowDecorations, WindowOptions
};

// --- Simple Custom Title Bar Component ---
struct SimpleTitleBar;

impl Render for SimpleTitleBar {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Basic styling inspired by Zed's title bar
        let title_bar_height = px(34.0); // Approximate height
        let title_bar_bg = rgb(0x2a2a2a); // Dark background
        let title_bar_text_color = rgb(0xcccccc); // Light text

        div()
            .id("simple-titlebar")
            .w_full() // Take full width
            .h(title_bar_height) // Set fixed height
            .bg(title_bar_bg) // Set background color
            // Apply top rounding if the window isn't tiled at the top corners
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.left)), |el| {
                el.rounded_tl_lg()
            })
            .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.right)), |el| {
                el.rounded_tr_lg()
            })
            .flex() // Use flexbox for internal layout
            .items_center() // Center items vertically
            .pl_2() // Add left padding
            .child(
                // Placeholder for title text
                div()
                    .child("Control")
                    .text_color(title_bar_text_color)
            )
            // Make the title bar draggable
            .on_mouse_down(gpui::MouseButton::Left, |_, win, _| {
                win.start_window_move(); // Allows dragging the window by the title bar
            })
    }
}


// --- Main Application View ---
struct MainApp {
    text: SharedString,
}

impl Render for MainApp {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Main container using vertical flex to stack title bar and content
        div()
            .flex()
            .flex_col()
            .size_full() // Fill the entire window
            .child(_cx.new(|_| SimpleTitleBar {}))
           
            .child(
                // Original content area
                div()
                    .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.left)), |el| {
                        el.rounded_bl_lg()
                    })
                    .when(matches!(window.window_decorations(), Decorations::Client { tiling, .. } if !(tiling.top || tiling.right)), |el| {
                        el.rounded_br_lg()
                    })
                    .bg(rgb(0x505050)) // Background for the content area
                    .flex_grow() // Allow this area to fill the remaining vertical space
                    .w_full() // Take full width
                    .flex() // Use flex to center the text
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(format!("Hello, {}!", &self.text))
            )
    }
}

// --- Application Entry Point ---
fn main() {
    Application::new().run(|cx: &mut crate::App| {
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
                window_min_size: Some(initial_size),
                app_id: Some("Control".into()),
                window_decorations: Some(WindowDecorations::Client), // IMPORTANT: Enable client-side decorations
                // -------------------------------------
                ..Default::default()
            },
            |_, cx| {
                // Create the main application view
                cx.new(|_| MainApp {
                    text: "World".into(),
                })
            },
        )
        .unwrap();

        cx.activate(true); // Activate the application
    });
}

