use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

use crate::app::view::View;

#[derive(Serialize, Deserialize)]
pub struct Home;

impl View for Home {
    fn name(&self) -> &str {
        "🏡 主页"
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.heading("欢迎来到我的主页");
    }
}
