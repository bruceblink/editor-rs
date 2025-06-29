use eframe::egui::{self, TopBottomPanel};

pub fn menu_example(ctx: &egui::Context) {
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
                    // 处理 Open
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    // 退出应用
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