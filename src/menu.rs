use eframe::egui::{self, ViewportCommand};
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
                    editor.picked_path = Some(path.display().to_string());
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