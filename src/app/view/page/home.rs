use eframe::egui::Ui;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::app::view::View;

#[derive(Serialize, Deserialize, Debug)]
pub struct Home;

impl View for Home {
    fn name(&self) -> &str {
        "🏡 主页"
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.heading("欢迎来到我的主页");
    }

    fn any(&self) -> &dyn Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
