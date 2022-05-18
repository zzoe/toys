use exam_builder::ui::ExamBuilder;
use serde::{Deserialize, Serialize};

use self::page::{Page, font_book::FontBook};

pub mod header;
pub mod menu;
pub mod page;

#[derive(Serialize, Deserialize)]
pub struct Toy {
    pub menu: menu::Menu,
    pub menu_switch: bool,
    pub header: header::Header,
    pub pages: Vec<Page>,
    pub current_page: usize,
}

impl Default for Toy {
    fn default() -> Self {
        Toy {
            menu: menu::Menu::default(),
            menu_switch: true,
            header: header::Header::default(),
            pages: vec![Page::ExamBuilder(ExamBuilder::default()),Page::FontBook(FontBook::default()),Page::ExamBuilder(ExamBuilder::default())],
            current_page: 0,
        }
    }
}

