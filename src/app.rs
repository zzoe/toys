use eframe::egui::{FontFamily, TextStyle};
use eframe::epaint::FontId;
use eframe::{egui, Frame, Storage};
use serde::{Deserialize, Serialize};
use toys::ui::{header, menu, page, Toy};

const VERSION: usize = 1;

#[derive(Serialize, Deserialize, Default)]
pub struct App {
    version: usize,
    toy: Toy,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.version != VERSION {
            self.version = VERSION;
            self.toy = Toy::default();
        }

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                header::view(&mut self.toy, ui);
            });

        if self.toy.menu_switch {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show(ctx, |ui| {
                    menu::view(&mut self.toy, ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            page::view(&mut self.toy, ui);
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

    let entry = fonts.families.entry(FontFamily::Proportional).or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    let entry = fonts.families.entry(FontFamily::Monospace).or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);

    // Get current context style
    let mut style = (*ctx.style()).clone();

    // Redefine text_styles
    style.text_styles = [
        (TextStyle::Small, FontId::new(16.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(20.0, FontFamily::Monospace)),
        (
            TextStyle::Monospace,
            FontId::new(20.0, FontFamily::Monospace),
        ),
        (TextStyle::Button, FontId::new(20.0, FontFamily::Monospace)),
        (TextStyle::Heading, FontId::new(36.0, FontFamily::Monospace)),
    ]
    .into();

    // Mutate global style with above changes
    ctx.set_style(style);
}
