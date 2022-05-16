use eframe::egui;
use eframe::egui::Widget;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Header {
    pub(crate) menu_switch: bool,
    #[serde(skip)]
    logo: Option<egui_extras::RetainedImage>,
}

impl Header {
    pub fn init(&mut self) {
        if self.logo.is_none() {
            self.logo = egui_extras::RetainedImage::from_svg_bytes(
                "rustacean-flat-happy.svg",
                include_bytes!("../../resource/rustacean-flat-happy.svg"),
            )
            .ok();
        }
    }

    pub fn view(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            // .min_height(32.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        if let Some(logo) = &mut self.logo {
                            if egui::ImageButton::new(logo.texture_id(ctx), egui::vec2(48.0, 30.0))
                                .ui(ui)
                                .clicked()
                            {
                                self.menu_switch = !self.menu_switch;
                            };
                        }

                        ui.heading("Expandable Upper Panel");
                    });
                });
            });
    }
}
