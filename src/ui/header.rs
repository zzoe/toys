use eframe::egui::{self, Ui};
use eframe::egui::{Image, Sense};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Header {
    pub menu_switch: bool,
    #[serde(skip)]
    logo: Option<egui_extras::RetainedImage>,
}

impl Header {
    pub fn view(&mut self, ui: &mut Ui) {
        ui.horizontal_centered(|ui| {
            if self.logo.is_none() {
                self.logo = egui_extras::RetainedImage::from_svg_bytes(
                    "rustacean-flat-happy.svg",
                    include_bytes!("../../resource/rustacean-flat-happy.svg"),
                )
                .ok();
            }

            if let Some(logo) = self.logo.as_ref() {
                let img = Image::new(logo.texture_id(ui.ctx()), egui::vec2(48.0, 30.0))
                    .sense(Sense::click());
                if ui.add(img).clicked() {
                    self.menu_switch = !self.menu_switch;
                }
            };

            ui.heading("Expandable Upper Panel");
        });
    }
}
