use eframe::egui::Ui;

pub mod header;
pub mod menu;
pub mod page;

pub trait View {
    fn name(&self) -> &str;
    fn view(&mut self, ui: &mut Ui);
}
