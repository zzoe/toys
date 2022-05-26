use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Home;

impl Home {
    pub fn view(&self, ui: &mut Ui) {
        ui.heading("欢迎来到我的主页");
    }
}
