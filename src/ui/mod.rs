use crate::ui::menu::Menu;
use eframe::egui;
use exam_builder::ui::ExamBuilder;
use serde::{Deserialize, Serialize};

pub mod menu;

#[derive(Serialize, Deserialize, Default)]
pub struct ToyUI {
    menu: Menu,
    exam_builder: ExamBuilder,
}

impl ToyUI {
    pub fn view(&mut self, ctx: &egui::Context) {
        self.menu.view(ctx);

        egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Expandable Upper Panel");
                    });
                    lorem_ipsum(ui);
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                lorem_ipsum(ui);
            });
            self.exam_builder.view(ctx);
        });
    }
}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label(egui::RichText::new("123、123、123").small().weak());
        },
    );
}
