use std::path::PathBuf;
use std::sync::Arc;

use dxtr_imgs_app::{EnglishTranslator, MessageKey, Translator};
use dxtr_imgs_domain::{Workplace, DEFAULT_WORKPLACE_NAME};
use dxtr_imgs_platform::{DesktopFileDialog, PlatformFileDialog};
use gpui::{
    div, img, point, prelude::*, px, rgb, size, App, Bounds, Context, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PinchEvent, Pixels, Point, Render, RenderImage,
    ScrollDelta, ScrollWheelEvent, Window, WindowBounds, WindowOptions,
};
use gpui_platform::application;
use tracing::info;
use tracing_subscriber::EnvFilter;

const VIEWPORT_WIDTH: f32 = 860.0;
const VIEWPORT_HEIGHT: f32 = 600.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 8.0;
const SCROLL_LINE_MULTIPLIER: f32 = 20.0;

struct ViewportState {
    image_path: Option<PathBuf>,
    image_dimensions: Option<(u32, u32)>,
    render_image: Option<Arc<RenderImage>>,
    zoom: f32,
    pan_offset: Point<Pixels>,
    last_mouse_position: Option<Point<Pixels>>,
    status: String,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            image_path: None,
            image_dimensions: None,
            render_image: None,
            zoom: 1.0,
            pan_offset: Point::default(),
            last_mouse_position: None,
            status: "Open a raster image or current RAW embedded preview.".to_owned(),
        }
    }
}

struct DesktopShell {
    workplace: Workplace,
    translator: EnglishTranslator,
    file_dialog: DesktopFileDialog,
    viewport: ViewportState,
}

impl DesktopShell {
    fn toolbar_button(
        id: &'static str,
        label: impl Into<String>,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Self, &mut Context<Self>) + 'static,
    ) -> impl IntoElement {
        div()
            .id(id)
            .px_3()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(rgb(0x3c414a))
            .bg(rgb(0x24272d))
            .text_sm()
            .cursor_pointer()
            .child(label.into())
            .active(|this| this.opacity(0.75))
            .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
    }

    fn fit_zoom_for(width: u32, height: u32) -> f32 {
        let scale_x = VIEWPORT_WIDTH / width.max(1) as f32;
        let scale_y = VIEWPORT_HEIGHT / height.max(1) as f32;
        scale_x.min(scale_y).min(1.0)
    }

    fn open_image(&mut self, cx: &mut Context<Self>) {
        let path = match self.file_dialog.pick_image_file() {
            Ok(Some(path)) => path,
            Ok(None) => return,
            Err(error) => {
                self.viewport.status = format!("File dialog failed: {error}");
                cx.notify();
                return;
            }
        };

        match dxtr_imgs_raw_engine::develop_preview(&path) {
            Ok(developed) => {
                let width = developed.width;
                let height = developed.height;
                let expected_len = developed.expected_rgba_len();
                let actual_len = developed.data.len();

                if expected_len != Some(actual_len) {
                    self.viewport.status = format!(
                        "Engine returned invalid RGBA buffer: expected {expected_len:?}, got {actual_len} bytes"
                    );
                    cx.notify();
                    return;
                }

                let mut pixels = developed.data;
                for pixel in pixels.chunks_exact_mut(4) {
                    pixel.swap(0, 2);
                }

                let Some(buffer) = image::RgbaImage::from_raw(width, height, pixels) else {
                    self.viewport.status = "Unable to adopt raw-engine RGBA buffer.".to_owned();
                    cx.notify();
                    return;
                };

                self.viewport.image_path = Some(path.clone());
                self.viewport.image_dimensions = Some((width, height));
                self.viewport.render_image =
                    Some(Arc::new(RenderImage::new(vec![image::Frame::new(buffer)])));
                self.viewport.zoom = Self::fit_zoom_for(width, height);
                self.viewport.pan_offset = Point::default();
                self.viewport.last_mouse_position = None;
                self.viewport.status = format!(
                    "{} — {} × {} — direct Rust raw-engine buffer",
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("image"),
                    width,
                    height
                );
            }
            Err(error) => {
                self.viewport.status = format!("raw-engine preview failed: {error}");
            }
        }

        cx.notify();
    }

    fn fit(&mut self, cx: &mut Context<Self>) {
        if let Some((width, height)) = self.viewport.image_dimensions {
            self.viewport.zoom = Self::fit_zoom_for(width, height);
            self.viewport.pan_offset = Point::default();
            cx.notify();
        }
    }

    fn one_to_one(&mut self, cx: &mut Context<Self>) {
        if self.viewport.render_image.is_some() {
            self.viewport.zoom = 1.0;
            self.viewport.pan_offset = Point::default();
            cx.notify();
        }
    }

    fn zoom_by(&mut self, factor: f32, cx: &mut Context<Self>) {
        if self.viewport.render_image.is_some() {
            self.viewport.zoom = (self.viewport.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
            cx.notify();
        }
    }

    fn is_dragging(&self) -> bool {
        self.viewport.last_mouse_position.is_some()
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button == MouseButton::Left || event.button == MouseButton::Middle {
            self.viewport.last_mouse_position = Some(event.position);
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.viewport.last_mouse_position = None;
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(last_position) = self.viewport.last_mouse_position {
            self.viewport.pan_offset += event.position - last_position;
            self.viewport.last_mouse_position = Some(event.position);
            cx.notify();
        }
    }

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.viewport.render_image.is_none() {
            return;
        }

        if event.modifiers.control || event.modifiers.platform {
            let delta: f32 = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.y.into(),
                ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_MULTIPLIER,
            };
            let zoom_factor = if delta > 0.0 {
                1.0 + delta.abs() * 0.01
            } else {
                1.0 / (1.0 + delta.abs() * 0.01)
            };
            self.viewport.zoom =
                (self.viewport.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
        } else {
            let delta = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels,
                ScrollDelta::Lines(lines) => lines.map(|value| px(value * SCROLL_LINE_MULTIPLIER)),
            };
            self.viewport.pan_offset += delta;
        }

        cx.notify();
    }

    fn handle_pinch(
        &mut self,
        event: &PinchEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.viewport.render_image.is_some() {
            self.viewport.zoom =
                (self.viewport.zoom * (1.0 + event.delta)).clamp(MIN_ZOOM, MAX_ZOOM);
            cx.notify();
        }
    }

    fn viewport_content(&self) -> impl IntoElement {
        let (Some(image), Some((width, height))) = (
            self.viewport.render_image.clone(),
            self.viewport.image_dimensions,
        ) else {
            return div()
                .size_full()
                .flex()
                .flex_col()
                .gap_2()
                .items_center()
                .justify_center()
                .child("Image viewport")
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x858a94))
                        .child("Open a raster or RAW file to render an owned raw-engine buffer"),
                )
                .into_any_element();
        };

        let scaled_width = width as f32 * self.viewport.zoom;
        let scaled_height = height as f32 * self.viewport.zoom;
        let left = px((VIEWPORT_WIDTH - scaled_width) / 2.0) + self.viewport.pan_offset.x;
        let top = px((VIEWPORT_HEIGHT - scaled_height) / 2.0) + self.viewport.pan_offset.y;

        div()
            .relative()
            .size_full()
            .child(
                div()
                    .absolute()
                    .left(left)
                    .top(top)
                    .w(px(scaled_width))
                    .h(px(scaled_height))
                    .child(img(image).w(px(scaled_width)).h(px(scaled_height))),
            )
            .into_any_element()
    }
}

impl Render for DesktopShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tr = &self.translator;
        let zoom_label = format!("{}%", (self.viewport.zoom * 100.0).round() as i32);

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
                    .child("M1 — direct Rust raw-engine viewport"),
            )
            .child(
                div()
                    .h(px(48.0))
                    .px_4()
                    .flex()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(rgb(0x2b2e34))
                    .bg(rgb(0x17191d))
                    .child(Self::toolbar_button("open-image", "Open Image", cx, |this, cx| {
                        this.open_image(cx);
                    }))
                    .child(Self::toolbar_button("fit", "Fit", cx, |this, cx| {
                        this.fit(cx);
                    }))
                    .child(Self::toolbar_button("one-to-one", "1:1", cx, |this, cx| {
                        this.one_to_one(cx);
                    }))
                    .child(Self::toolbar_button("zoom-out", "−", cx, |this, cx| {
                        this.zoom_by(1.0 / 1.2, cx);
                    }))
                    .child(Self::toolbar_button("zoom-in", "+", cx, |this, cx| {
                        this.zoom_by(1.2, cx);
                    }))
                    .child(div().ml_2().text_sm().text_color(rgb(0x8d929c)).child(zoom_label)),
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
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_3()
                            .bg(rgb(0x101114))
                            .child(
                                div()
                                    .w(px(VIEWPORT_WIDTH))
                                    .h(px(VIEWPORT_HEIGHT))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(rgb(0x343840))
                                    .bg(rgb(0x202329))
                                    .overflow_hidden()
                                    .cursor(if self.is_dragging() {
                                        gpui::CursorStyle::ClosedHand
                                    } else {
                                        gpui::CursorStyle::OpenHand
                                    })
                                    .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                                    .on_pinch(cx.listener(Self::handle_pinch))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(Self::handle_mouse_down),
                                    )
                                    .on_mouse_down(
                                        MouseButton::Middle,
                                        cx.listener(Self::handle_mouse_down),
                                    )
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(Self::handle_mouse_up),
                                    )
                                    .on_mouse_up(
                                        MouseButton::Middle,
                                        cx.listener(Self::handle_mouse_up),
                                    )
                                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                                    .child(self.viewport_content()),
                            )
                            .child(
                                div()
                                    .w(px(VIEWPORT_WIDTH))
                                    .text_sm()
                                    .text_color(rgb(0x858a94))
                                    .child(self.viewport.status.clone()),
                            ),
                    )
                    .child(
                        div()
                            .w(px(280.0))
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .border_l_1()
                            .border_color(rgb(0x2b2e34))
                            .bg(rgb(0x17191d))
                            .child(tr.tr(MessageKey::Develop))
                            .child("Exposure        0.00")
                            .child("Temperature     0.00")
                            .child("Contrast        1.00")
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x858a94))
                                    .child("M1 keeps existing processing scope only."),
                            ),
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
    info!("starting Dextryx Images M1 desktop shell");

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
                    file_dialog: DesktopFileDialog,
                    viewport: ViewportState::default(),
                })
            },
        )
        .expect("failed to open Dextryx Images window");
        cx.activate(true);
    });

    debug_assert_eq!(DEFAULT_WORKPLACE_NAME, "My workplace");
}
