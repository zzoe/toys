use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_channel::Sender;
use eframe::egui::{self, Ui};
use eframe::egui::{Image, Sense};
use egui_extras::RetainedImage;

use crate::app::view::View;
use crate::app::worker::header::LogoClick;
use crate::app::worker::Task;

pub struct Header {
    sender: Sender<Arc<dyn Task>>,
    logo: RetainedImage,
}

impl Header {
    pub fn new(sender: Sender<Arc<dyn Task>>) -> Self {
        Self {
            sender,
            logo: RetainedImage::from_svg_bytes(
                "rustacean-flat-happy.svg",
                include_bytes!("../../../resource/rustacean-flat-happy.svg"),
            )
            .unwrap(),
        }
    }
}

impl View for Header {
    fn name(&self) -> &str {
        "header"
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal_centered(|ui| {
            let img = Image::new(self.logo.texture_id(ui.ctx()), egui::vec2(48.0, 30.0))
                .sense(Sense::click());
            if ui.add(img).clicked() {
                self.sender.send_blocking(Arc::new(LogoClick {})).ok();
            }

            ui.heading("Toys");
        });
    }

    fn any(&self) -> &dyn Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("sender").finish()
    }
}
