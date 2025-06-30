use eframe::egui;
use eframe::egui::{TopBottomPanel, ViewportCommand};

#[derive(Clone, Copy, Default)]
pub struct TitleBarPanel {

}

impl TitleBarPanel {

    pub fn title_bar_ui(self, ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {

        use egui::{vec2, Align2, FontId, Id, PointerButton, Sense, UiBuilder};

        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            Id::new("title_bar"),
            Sense::click_and_drag(),
        );

        // Paint the title:
        painter.text(
            title_bar_rect.center(),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        // Paint the line under the title:
        painter.line_segment(
            [
                title_bar_rect.left_bottom() + vec2(1.0, 0.0),
                title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        // Interact with the title bar (drag to move window):
        if title_bar_response.double_clicked() {
            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        if title_bar_response.drag_started_by(PointerButton::Primary) {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }

        ui.scope_builder(
            UiBuilder::new()
                .max_rect(title_bar_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);
                self.close_maximize_minimize(ui);
            },
        );
    }


    /// Show some close/maximize/minimize buttons for the native window.
    fn close_maximize_minimize(self, ui: &mut egui::Ui) {
        use egui::{Button, RichText};

        let button_height = 20.0;

        let close_response = ui
            .add(Button::new(RichText::new("❌").size(button_height)))
            .on_hover_text("Close the window");
        if close_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        if is_maximized {
            let maximized_response = ui
                .add(Button::new(RichText::new("🗗").size(button_height)))
                .on_hover_text("Restore window");
            if maximized_response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(false));
            }
        } else {
            let maximized_response = ui
                .add(Button::new(RichText::new("🗗").size(button_height)))
                .on_hover_text("Maximize window");
            if maximized_response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
            }
        }

        let minimized_response = ui
            .add(Button::new(RichText::new("🗕").size(button_height)))
            .on_hover_text("Minimize window");
        if minimized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
        }
    }

    // 构建title_bar
    pub fn title_bar(self, ctx: &egui::Context) {
        let title_frame = self.custom_title_bar_frame(ctx);

        TopBottomPanel::top("title_bar_panel").frame(title_frame).exact_height(32.0).show(ctx, |ui| {
            let rect = ui.max_rect();
            self.title_bar_ui(ui, rect, "Editor-rs");
        });
    }

    fn custom_title_bar_frame(self, ctx: &egui::Context) -> egui::Frame {
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
    
}

