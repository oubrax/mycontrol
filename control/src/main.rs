use anyhow::Result;
use gpui::{App, AppContext, Application, WindowKind, WindowOptions};
use tokio::task;
use ui::{pages::AskPage, ControlRoot};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    task::spawn_blocking(move || {
        Application::new().run(move |cx: &mut App| {
            gpui_component::init(cx);
            cx.open_window(
                WindowOptions {
                    kind: WindowKind::PopUp,
                    ..Default::default()
                },
                |win, cx| {
                    let main_view = cx.new(|cx| AskPage::new(cx));
                    let main = cx.new(|cx| ControlRoot::new(cx, main_view.into()));

                    main
                },
            )
            .expect("Could not open window");
        })
    })
    .await?;

    Ok(())
}
