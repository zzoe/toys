use eframe::egui;
use eframe::egui::{Label, Sense, Ui};
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
    pub fn view(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
            ui.spacing_mut().indent_ends_with_horizontal_line = true;
            ui.add_space(10.0);

            self.add_search(ui);
            self.add_home(ui);
            ui.separator();
        });
    }

    fn add_search(&mut self, ui: &mut Ui) {
        ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("搜索"));
    }

    fn add_home(&mut self, ui: &mut Ui) {
        let label = Label::new("🏡 主页").sense(Sense::click());
        if ui.add(label).clicked() {
            println!("clicked");
        }
    }
}
