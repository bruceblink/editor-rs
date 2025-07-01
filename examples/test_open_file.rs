// Cargo.toml 需要依赖：
// rfd = "0.9"       // 用于跨平台的文件对话框

use eframe::egui;
use eframe::egui::{FontDefinitions, FontFamily};
use egui::ScrollArea;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
//use editor_rs::editor_app::EditorApp;

struct MyApp {
    file_content: String,
    current_file: Option<PathBuf>, // 新增：记录当前文件路径
}

impl MyApp {
    // 新增：构造函数
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_chinese_font(&cc.egui_ctx);
        Self {
            file_content: String::new(),
            current_file: None,
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_content: String::new(),
            current_file: None,
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame:  &mut eframe::Frame) {
        // 捕获键盘事件
        let input = ctx.input(|input| input.clone());

        // 检查是否按下 Ctrl + O 打开文件
        if input.key_pressed(egui::Key::O) && input.modifiers.ctrl {
            self.open_file();
        }

        // 检查是否按下 Ctrl + S 保存文件
        if input.key_pressed(egui::Key::S) && input.modifiers.ctrl {
            self.save_file();
        }

        // 顶部工具栏：打开 + 保存
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 打开文件
                if ui.button("Open…").clicked() {
                    self.open_file();
                }

                // 保存文件
                if ui.button("Save").clicked() {
                    self.save_file();
                }
            });
        });
         // 禁用底部面板的空白
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // 此处可以放入一些状态显示或空的内容，避免占用过多空间
            ui.horizontal(|ui| {
                ui.label("状态栏：当前没有额外信息");
            });
        }); 
        // 中央面板：带滚动条的文本显示/编辑区
        // 获取中央面板的可用空间
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();

            // 显示当前文件名
            ui.label(
                self.current_file
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("未命名文件"),
            );

            ScrollArea::vertical()
                .max_height(available_height)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_sized(
                        [available_width, available_height],
                        egui::TextEdit::multiline(&mut self.file_content)
                            .font(egui::TextStyle::Monospace)
                            .lock_focus(false)
                    );
                });
        });
    }
}

impl MyApp {
    // 打开文件的函数
    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("文本文件", &["txt", "rs", "md"])
            .set_title("请选择一个文本文件")
            .pick_file()
        {
            match fs::read_to_string(&path) {
                Ok(text) => {
                    self.file_content = text;
                    self.current_file = Some(path);
                }
                Err(err) => {
                    self.file_content = format!("读取失败：{}", err);
                    self.current_file = None;
                }
            }
        }
    }

    // 保存文件的函数
    fn save_file(&mut self) {
        // 如果已经有打开的文件路径，直接写入
        if let Some(path) = &self.current_file {
            if let Err(err) = fs::write(path, &self.file_content) {
                self.file_content = format!("保存失败：{}", err);
            }
        } else {
            // 否则弹另存为对话框
            if let Some(path) = FileDialog::new()
                .set_title("另存为")
                .add_filter("文本文件", &["txt", "rs", "md"])
                .set_file_name("untitled.txt")
                .save_file()
            {
                if let Err(err) = fs::write(&path, &self.file_content) {
                    self.file_content = format!("保存失败：{}", err);
                } else {
                    self.current_file = Some(path);
                }
            }
        }
    }
}


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

fn main()  -> eframe::Result{
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(true)  // Hide the OS-specific "chrome" around the window
            .with_inner_size([1280.0, 1024.0])// Initial size of the window
            .with_drag_and_drop(true)  // wide enough for the drag-drop overlay text
            .with_resizable(true),  // Allow resizing the window
        ..Default::default()
    };
    eframe::run_native(
        "Editor-rs",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
    )
}
