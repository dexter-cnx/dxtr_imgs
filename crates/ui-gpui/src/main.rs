use dxtr_imgs_app::{EnglishTranslator, MessageKey, Translator};
use dxtr_imgs_domain::{Workplace, DEFAULT_WORKPLACE_NAME};
use gpui::{
    div, prelude::*, px, rgb, size, App, Bounds, Context, Render, Window, WindowBounds,
    WindowOptions,
};
use gpui_platform::application;
use tracing::info;
use tracing_subscriber::EnvFilter;

struct DesktopShell {
    workplace: Workplace,
    translator: EnglishTranslator,
}

impl Render for DesktopShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let tr = &self.translator;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x121316))
            .text_color(rgb(0xe6e8ec))
            .child(
                div()
                    .h(px(44.0))
                    .px_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0x2b2e34))
                    .bg(rgb(0x191b1f))
                    .child(tr.tr(MessageKey::AppName))
                    .child("M0 — Rust / GPUI foundation"),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(
                        div()
                            .w(px(220.0))
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .border_r_1()
                            .border_color(rgb(0x2b2e34))
                            .bg(rgb(0x17191d))
                            .child(tr.tr(MessageKey::Workplace))
                            .child(self.workplace.name.clone())
                            .child("")
                            .child(tr.tr(MessageKey::Catalog))
                            .child(tr.tr(MessageKey::AllPhotos))
                            .child(tr.tr(MessageKey::Missing))
                            .child(tr.tr(MessageKey::RecentImports)),
                    )
                    .child(
                        div().flex_1().flex().items_center().justify_center().child(
                            div()
                                .text_color(rgb(0x8d929c))
                                .child("Catalog viewport foundation"),
                        ),
                    )
                    .child(
                        div()
                            .w(px(280.0))
                            .p_4()
                            .border_l_1()
                            .border_color(rgb(0x2b2e34))
                            .bg(rgb(0x17191d))
                            .child(tr.tr(MessageKey::Develop)),
                    ),
            )
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}

fn main() {
    init_tracing();
    info!("starting Dextryx Images desktop shell");

    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1440.0), px(960.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|_cx| DesktopShell {
                    workplace: Workplace::default_workplace(),
                    translator: EnglishTranslator,
                })
            },
        )
        .expect("failed to open Dextryx Images window");
        cx.activate(true);
    });

    debug_assert_eq!(DEFAULT_WORKPLACE_NAME, "My workplace");
}
