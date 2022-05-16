use eframe::egui;
use exam_builder::ui::ExamBuilder;
use serde::{Deserialize, Serialize};

pub mod header;
pub mod menu;

#[derive(Serialize, Deserialize, Default)]
pub struct ToyUI {
    menu: menu::Menu,
    header: header::Header,
    exam_builder: ExamBuilder,
}

impl ToyUI {
    pub fn init(&mut self) {
        self.header.init();
    }
    
    pub fn view(&mut self, ctx: &egui::Context) {
        self.header.view(ctx);

        if self.header.menu_switch {
            self.menu.view(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
            self.exam_builder.view(ctx);
        });
    }
}
