use crate::menu::build_menu_bar;
use crate::title_bar::TitleBarPanel;
use eframe::egui;
use eframe::egui::{FontDefinitions, FontFamily, ScrollArea, ViewportCommand};
use std::sync::Arc;

#[derive(Default)]
pub struct EditorApp {
    pub show_confirmation_dialog: bool,
    pub allowed_to_close: bool,
    pub dropped_files: Vec<egui::DroppedFile>,
    pub picked_path: Option<String>,
    pub file_content: String, // 文件内容
    pub title_bar: TitleBarPanel,
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 构建title bar
        self.title_bar.title_bar(ctx);
        // 构建 menu bar
        build_menu_bar(self, ctx);
        // 构建中央内容区
        self.build_central_panel(ctx);
        // 预览拖拽的文件
        preview_files_being_dropped(ctx);

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

impl EditorApp {
    
    pub fn default() -> Self {
        Self {
            show_confirmation_dialog: false,
            allowed_to_close: false,
            dropped_files: Vec::new(),
            picked_path: None,
            file_content: String::new(), // 初始化文件内容为空
            title_bar: TitleBarPanel::new("Editor-rs"),
        }
    }
    
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_chinese_font(&cc.egui_ctx);
        Self::default()
    }

    pub fn build_central_panel(&mut self, ctx: &egui::Context) {
        let _panel_frame = custom_central_panel_frame(ctx);
        egui::CentralPanel::default().frame(_panel_frame).show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.file_content)
                            .font(egui::TextStyle::Monospace) // 等宽字体
                            .desired_rows(20)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    );
                });

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
    }
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

// Preview hovering files:
pub fn preview_files_being_dropped(ctx: &egui::Context) {
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