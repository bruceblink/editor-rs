#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use editor_rs::EditorApp;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)  // Hide the OS-specific "chrome" around the window
            .with_inner_size([640.0, 480.0])// Initial size of the window
            .with_drag_and_drop(true)  // wide enough for the drag-drop overlay text
            .with_resizable(true),  // Allow resizing the window
        ..Default::default()
    };
    eframe::run_native(
        "Editor-rs",
        options,
        Box::new(|_cc| Ok(Box::<EditorApp>::default())),
    )
}