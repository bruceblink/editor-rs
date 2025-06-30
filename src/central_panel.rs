use eframe::egui;
use crate::editor_app::EditorApp;

pub fn build_central_panel(app: &mut EditorApp, ctx: &egui::Context) {
    let _panel_frame = custom_central_panel_frame(ctx);
    egui::CentralPanel::default().frame(_panel_frame).show(ctx, |ui| {
        ui.label("Drag-and-drop files onto the window!");
        if let Some(picked_path) = &app.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
            });
        }

        // Show dropped files (if any):
        if !app.dropped_files.is_empty() {
            ui.group(|ui| {
                ui.label("Dropped files:");

                for file in &app.dropped_files {
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