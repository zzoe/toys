use eframe::egui::{self, Ui};
use eframe::egui::{Image, Sense};
use serde::{Deserialize, Serialize};

use super::Toy;

#[derive(Serialize, Deserialize, Default)]
pub struct Header {
    #[serde(skip)]
    logo: Option<egui_extras::RetainedImage>,
}

pub fn view(toy: &mut Toy, ui: &mut Ui) {
    ui.horizontal_centered(|ui| {
        if toy.header.logo.is_none() {
            toy.header.logo = egui_extras::RetainedImage::from_svg_bytes(
                "rustacean-flat-happy.svg",
                include_bytes!("../../resource/rustacean-flat-happy.svg"),
            )
            .ok();
        }

        if let Some(logo) = toy.header.logo.as_ref() {
            let img =
                Image::new(logo.texture_id(ui.ctx()), egui::vec2(48.0, 30.0)).sense(Sense::click());
            if ui.add(img).clicked() {
                toy.menu_switch = !toy.menu_switch;
            }
        };

        ui.heading("Toys");
    });
}
