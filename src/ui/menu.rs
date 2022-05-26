use eframe::egui;
use eframe::egui::{Label, Sense, Ui};
use serde::{Deserialize, Serialize};

use super::Toy;

macro_rules! add_label {
    ($ui:tt, $toy:tt, $text:tt, $index:expr) => {
        let label = Label::new($text).sense(Sense::click());
        if $ui.add(label).clicked() {
            $toy.current_page = $index;
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Menu {
    search: String,
    menus: Vec<(String, usize)>,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            search: String::new(),
            menus: vec![
                ("🏡 主页".to_string(), 0),
                ("📖 Font Book".to_string(), 1),
                ("🖹 Exam Builder".to_string(), 2),
            ],
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
        ui.separator();

        toy.menu.menus[1..]
            .iter()
            .filter(|(label, _)| {
                if toy.menu.search.is_empty() {
                    true
                } else {
                    label
                        .to_ascii_lowercase()
                        .contains(&toy.menu.search.to_ascii_lowercase())
                }
            })
            .for_each(|(label, page_id)| {
                add_label!(ui, toy, label, *page_id);
            });
    });
}

fn add_search(toy: &mut Toy, ui: &mut Ui) {
    ui.add(egui::TextEdit::singleline(&mut toy.menu.search).hint_text("搜索"));
}
