use eframe::egui::{self, TopBottomPanel, ViewportCommand};
use crate::editor_app::EditorApp;

pub fn menu_example(editor: &mut EditorApp, ctx: &egui::Context) {
    // 在应用窗口的最顶端创建一个“菜单栏”面板
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        // 用 egui 提供的 menu::bar 来布局一排菜单按钮
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

                if let Some(picked_path) = &editor.picked_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
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
        
    });
}