use eframe::egui;
use eframe::egui::{Label, Sense, Ui};
use serde::{Deserialize, Serialize};

use super::Toy;

macro_rules! add_label {
    ($ui:tt, $toy:tt, $text:tt, $index:tt) => {
        let label = Label::new($text).sense(Sense::click());
        if $ui.add(label).clicked() {
            $toy.current_page = $index;
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Menu {
    search: String,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            search: String::new(),
        }
    }
}

pub fn view(toy: &mut Toy, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
        ui.spacing_mut().indent_ends_with_horizontal_line = true;
        ui.add_space(10.0);

        add_search(toy, ui);
        add_label!(ui, toy, "🏡 主页", 0);
        // add_home(toy, ui);
        ui.separator();
        add_label!(ui, toy, "Font Book", 1);
        add_label!(ui, toy, "Exam Builder", 2);
    });
}

fn add_search(toy: &mut Toy, ui: &mut Ui) {
    ui.add(egui::TextEdit::singleline(&mut toy.menu.search).hint_text("搜索"));
}