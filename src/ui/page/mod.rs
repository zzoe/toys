use eframe::egui::Ui;
use exam_builder::ui::ExamBuilder;
use serde::{Deserialize, Serialize};

use super::Toy;
use font_book::FontBook;
use home::Home;

pub mod font_book;
pub mod home;

#[derive(Serialize, Deserialize)]
pub enum Page {
    Home(Home),
    ExamBuilder(ExamBuilder),
    FontBook(FontBook),
}

pub fn view(toy: &mut Toy, ui: &mut Ui) {
    if let Some(page) = toy.pages.get_mut(toy.current_page) {
        match page {
            Page::Home(home) => home.view(ui),
            Page::ExamBuilder(exam_builder) => exam_builder.view(ui),
            Page::FontBook(font_book) => font_book.view(ui),
        }
    }
}
