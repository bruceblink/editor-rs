use crate::central_panel::{build_central_panel, preview_files_being_dropped};
use crate::menu::build_menu_bar;
use crate::title_bar::TitleBarPanel;
use eframe::egui;
use eframe::egui::{FontDefinitions, FontFamily, ViewportCommand};
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
        build_central_panel(self, ctx);
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
    
}