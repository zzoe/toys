use eframe::{egui, Frame, Storage};
use serde::{Deserialize, Serialize};
use toys::ui::ToyUI;

#[derive(Serialize, Deserialize, Default)]
pub struct App {
    toy: ToyUI,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            // .min_height(32.0)
            .show(ctx, |ui| {
                self.toy.header.view(ui);
            });

        if self.toy.header.menu_switch {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show(ctx, |ui| {
                    self.toy.menu.view(ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.toy.exam_builder.view(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // cc.egui_ctx.set_debug_on_hover(true);

        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "consola".to_owned(),
        egui::FontData::from_static(include_bytes!("../resource/consola.ttf")),
    );
    fonts.font_data.insert(
        "simkai".to_owned(),
        egui::FontData::from_static(include_bytes!("../resource/simkai.ttf")),
    );

    let entry = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    let entry = fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
