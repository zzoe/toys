use exam_builder::ui::ExamBuilder;
use serde::{Deserialize, Serialize};

pub mod header;
pub mod menu;

#[derive(Serialize, Deserialize, Default)]
pub struct ToyUI {
    pub menu: menu::Menu,
    pub header: header::Header,
    pub exam_builder: ExamBuilder,
}
