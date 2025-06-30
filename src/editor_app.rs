use crate::menu::menu_example;
use crate::title_bar::title_bar_ui;
use eframe::egui;
use eframe::egui::{FontDefinitions, FontFamily, Id, LayerId, Order, Stroke, TopBottomPanel, ViewportCommand};
use std::sync::Arc;

#[derive(Default)]
pub struct EditorApp {
    pub(crate) show_confirmation_dialog: bool,
    pub(crate) allowed_to_close: bool,
    dropped_files: Vec<egui::DroppedFile>,
    pub(crate) picked_path: Option<String>,
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set a custom Chinese font for the application.
        set_chinese_font(ctx);
        let title_frame = custom_title_bar_frame(ctx);
        // 顶部 title_bar
        TopBottomPanel::top("title_bar_panel").frame(title_frame).exact_height(32.0).show(ctx, |ui| {
            let rect = ui.max_rect();
            title_bar_ui(ui, rect, "Editor-rs");
        });
        // 顶部 menu_bar，紧跟 title_bar 之下
        TopBottomPanel::top("menu_bar_panel").exact_height(24.0).show(ctx, |ui| {
            menu_example(self, ui);
            let rect   = ui.clip_rect();
            let color  = ctx.style().visuals.widgets.noninteractive.fg_stroke.color;
            let stroke = Stroke::new(2.0, color);  // 把宽度设为 2

            let painter = ui.painter();
            painter.line_segment([rect.left_top(),    rect.left_bottom()],  stroke);
            painter.line_segment([rect.right_top(),   rect.right_bottom()], stroke);
        });

        let _panel_frame = custom_central_panel_frame(ctx);

        egui::CentralPanel::default().frame(_panel_frame).show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .current_pos(ctx.screen_rect().center())
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                });
        }
        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }
}

/** * Set a custom Chinese font for the application.
 * This function is called to ensure that the application can display Chinese characters correctly.
 */
fn set_chinese_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 加载自定义中文字体
    fonts.font_data.insert(
        "my_chinese".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../fonts/simsun.ttc"))), // 路径根据实际情况调整
    );

    // 将自定义字体加入到 Proportional 和 Monospace 字体族的最前面
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_chinese".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_chinese".to_owned());

    ctx.set_fonts(fonts);
}

// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}



fn custom_title_bar_frame(ctx: &egui::Context) -> egui::Frame {
    use egui::CornerRadius;
    let mut rounding = CornerRadius::ZERO;
    rounding.nw = 10.0 as u8; // 右上角
    rounding.ne = 10.0 as u8; // 右下角

    egui::Frame::NONE
        .fill(ctx.style().visuals.window_fill())
        //.stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke)
        .corner_radius(rounding)
        .outer_margin(1.0)
}

fn custom_central_panel_frame(ctx: &egui::Context) -> egui::Frame {
    use egui::CornerRadius;
    let mut rounding = CornerRadius::ZERO;
    rounding.sw = 10.0 as u8; // 左下角
    rounding.se = 10.0 as u8; // 右下角

    egui::Frame::NONE
        .fill(ctx.style().visuals.window_fill())
        .corner_radius(rounding)
        .outer_margin(1.0)
}