use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, CreationContext};
use std::sync::Arc;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tab {
    Home,
    Editor,
    Settings,
}

pub struct MyApp {
    current_tab: Tab,
    editor_content: String,
}

impl MyApp {

    fn default() -> Self {
        Self {
            current_tab: Tab::Home,
            editor_content: String::from("这是编辑器内容，支持多行编辑..."),
        }
    }
    
    fn new(cc: &CreationContext) -> Self {
        set_chinese_font(&cc.egui_ctx);
        Self::default()
    }
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部栏：显示标签页按钮
        egui::TopBottomPanel::top("top_panel").exact_height(40.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // tab 按钮
                for &tab in &[Tab::Home, Tab::Editor, Tab::Settings] {
                    let selected = self.current_tab == tab;
                    if ui.selectable_label(selected, format!("{:?}", tab)).clicked() {
                        self.current_tab = tab;
                    }
                }
            });
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("bottom_panel").exact_height(24.0).show(ctx, |ui| {
            ui.label("状态: 运行中");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("v0.1.0");
            });
        });

        // 左侧导航栏（示例，这里简单显示当前 tab）
        egui::SidePanel::left("side_panel").default_width(180.0).show(ctx, |ui| {
            ui.heading("导航栏");
            ui.separator();
            ui.label(format!("当前标签: {:?}", self.current_tab));
        });

        // 中央主内容区域，根据当前 tab 展示不同内容
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Home => {
                    ui.heading("首页");
                    ui.label("欢迎访问首页内容。");
                }
                Tab::Editor => {
                    ui.heading("编辑器");
                    ui.label("请输入内容：");
                    ui.add(egui::TextEdit::multiline(&mut self.editor_content).desired_rows(20));
                }
                Tab::Settings => {
                    ui.heading("设置");
                    ui.label("这里是设置页。");
                }
            }
        });
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("多标签 Egui 应用", 
                       options,
                       Box::new(|_cc| Ok(Box::new(MyApp::new(_cc))))
    )
}
