use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use async_channel::{Receiver, Sender};
use eframe::egui::{Align, FontFamily, Layout, TextStyle};
use eframe::epaint::FontId;
use eframe::{egui, Frame, Storage};

use config::ToyConfig;

use crate::app::view::header::Header;
use crate::app::view::menu::Menu;
use crate::app::view::View;
use crate::app::worker::{Event, Task};

pub mod config;
pub mod error;
pub mod view;
pub mod worker;

pub struct Channel {
    task_sender: Sender<Arc<dyn Task>>,
    event_receiver: Receiver<Arc<dyn Event>>,
}

#[derive(Debug)]
pub struct App {
    pub cfg: ToyConfig,
    pub event_receiver: Receiver<Arc<dyn Event>>,
    pub header: Header,
    pub menu_switch: bool,
    pub menu: Menu,
    pub page: Rc<RefCell<dyn View>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        while let Ok(event) = self.event_receiver.try_recv() {
            event.handle(self);
        }

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.header.view(ui);
            });

        if self.menu_switch {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show(ctx, |ui| {
                    self.menu.view(ui);

                    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                        egui::widgets::global_dark_light_mode_switch(ui);
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.page.borrow_mut().view(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.cfg);
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let egui_ctx = cc.egui_ctx.clone();
        setup_custom_fonts(&egui_ctx);
        // egui_ctx.set_visuals(egui::Visuals::dark());
        // egui_ctx.set_debug_on_hover(true);

        let cfg = cc
            .storage
            .and_then(|storage| eframe::get_value::<ToyConfig>(storage, eframe::APP_KEY))
            .unwrap_or_default();

        let Channel {
            task_sender,
            event_receiver,
        } = worker::start(egui_ctx);

        let menu = Menu::new(task_sender.clone(), &cfg);
        let page = menu.home();

        Self {
            cfg,
            event_receiver,
            header: Header::new(task_sender),
            menu_switch: true,
            menu,
            page,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "jetbrains".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../resource/JetBrainsMonoNLNerdFontMono-Regular.ttf"
        )),
    );
    fonts.font_data.insert(
        "simkai".to_owned(),
        egui::FontData::from_static(include_bytes!("../../resource/simkai.ttf")),
    );

    let entry = fonts.families.entry(FontFamily::Proportional).or_default();
    entry.push("jetbrains".to_owned());
    entry.push("simkai".to_owned());

    let entry = fonts.families.entry(FontFamily::Monospace).or_default();
    entry.push("jetbrains".to_owned());
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
