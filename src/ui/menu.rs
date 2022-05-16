use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Menu {
    search: String,
    nav: Vec<String>,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            search: String::new(),
            nav: vec!["Home".to_string(), "About".to_string()],
        }
    }
}

impl Menu {
    pub fn view(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("搜索"));
                    if ui.button("🏡 主页").clicked() {}
                    ui.separator();
                });
            });
    }
}
