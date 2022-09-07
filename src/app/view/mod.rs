use eframe::egui::Ui;
use std::any::Any;
use std::fmt::Debug;

pub mod header;
pub mod menu;
pub mod page;

pub trait View: Debug {
    fn name(&self) -> &str;
    fn view(&mut self, ui: &mut Ui);
    fn any(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;
}
