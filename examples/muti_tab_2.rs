use eframe::{egui, egui::TextEdit, CreationContext};
use std::collections::HashMap;
use std::sync::Arc;
use eframe::egui::{FontDefinitions, FontFamily};

#[derive(Debug, Clone)]
struct Tab {
    id: usize,
    title: String,
    content: String,
}

pub struct MyApp {
    tabs: HashMap<usize, Tab>,
    order: Vec<usize>,
    active: Option<usize>,
    next_id: usize,
}

impl MyApp {

    fn default() -> Self {
        let mut tabs = HashMap::new();
        tabs.insert(0, Tab {
            id: 0,
            title: "新建标签".to_string(),
            content: "这里是内容".into(),
        });
        Self {
            tabs,
            order: vec![0],
            active: Some(0),
            next_id: 1,
        }
    }
    
    fn new(p0: &CreationContext) -> Self {
        set_chinese_font(&p0.egui_ctx);
        Self::default()
    }
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);

        // 顶部标签页栏
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let mut to_remove = None;
                for &id in &self.order {
                    let tab = self.tabs.get(&id).unwrap();
                    let is_active = Some(id) == self.active;

                    let label = if is_active {
                        format!("🔵 {}", tab.title)
                    } else {
                        tab.title.clone()
                    };

                    let button = egui::Button::new(label).small();
                    if ui.add(button).clicked() {
                        self.active = Some(id);
                    }

                    if ui.button("×").on_hover_text("关闭标签").clicked() {
                        to_remove = Some(id);
                    }
                }

                if ui.button("+").clicked() {
                    self.new_tab("未命名".into());
                }

                if let Some(id) = to_remove {
                    self.close_tab(id);
                }
            });
        });

        // 主编辑区
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab_id) = self.active {
                if let Some(tab) = self.tabs.get_mut(&tab_id) {
                    ui.add(TextEdit::multiline(&mut tab.content).desired_rows(20).desired_width(f32::INFINITY));
                }
            } else {
                ui.label("无标签页打开");
            }
        });
    }
}

impl MyApp {
    fn new_tab(&mut self, title: String) {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.insert(
            id,
            Tab {
                id,
                title,
                content: String::new(),
            },
        );
        self.order.push(id);
        self.active = Some(id);
    }

    fn close_tab(&mut self, id: usize) {
        self.tabs.remove(&id);
        self.order.retain(|&x| x != id);
        if self.active == Some(id) {
            self.active = self.order.last().copied();
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::O)) {
            self.new_tab("打开的文件".into()); // 模拟打开
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
            if let Some(id) = self.active {
                if let Some(tab) = self.tabs.get(&id) {
                    println!("保存 [{}]: {}", tab.title, tab.content);
                }
            }
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::W)) {
            if let Some(id) = self.active {
                self.close_tab(id);
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("多标签 Egui 应用",
                       options,
                       Box::new(|_cc| Ok(Box::new(MyApp::new(_cc))))
    )
}