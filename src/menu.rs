use std::fs;
use eframe::egui::{self, Stroke, TopBottomPanel, ViewportCommand};
use crate::editor_app::EditorApp;

pub fn menu_example(editor: &mut EditorApp, ui: &mut egui::Ui) {
    // 只渲染菜单栏内容，不包裹TopBottomPanel
    egui::menu::bar(ui, |ui| {
        // File 菜单
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                // 处理 New
            }
            if ui.button("Open…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    // 读取文件内容到字符串
                    match fs::read_to_string(&path) {
                        Ok(text) => editor.file_content = text.to_string(),
                        Err(err) => editor.file_content = format!("读取失败：{}", &err),
                    }
                }
            }


            ui.separator();
            if ui.button("Quit").clicked() {
                // 退出应用
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }
        });

        // Edit 菜单
        ui.menu_button("Edit", |ui| {
            if ui.button("Undo").clicked() {
                // 处理 Undo
            }
            if ui.button("Redo").clicked() {
                // 处理 Redo
            }
        });

        // Help 菜单
        ui.menu_button("Help", |ui| {
            ui.horizontal(|ui| {
                ui.label("theme:");
                egui::widgets::global_theme_preference_buttons(ui);
            });
            if ui.button("About…").clicked() {
                // 弹出 About 对话框

            }
        });
    });
}

pub fn build_menu_bar(app: &mut EditorApp ,ctx: &egui::Context) {
    // 顶部 menu_bar，紧跟 title_bar 之下
    TopBottomPanel::top("menu_bar_panel").exact_height(24.0).show(ctx, |ui| {
        menu_example(app, ui);
        let rect   = ui.clip_rect();
        let color  = ctx.style().visuals.widgets.noninteractive.fg_stroke.color;
        let stroke = Stroke::new(2.0, color);  // 把宽度设为 2

        let painter = ui.painter();
        painter.line_segment([rect.left_top(),    rect.left_bottom()],  stroke);
        painter.line_segment([rect.right_top(),   rect.right_bottom()], stroke);
    });
}