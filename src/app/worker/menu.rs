use std::sync::Arc;

use crate::app::worker::{Event, Task};
use crate::app::App;

#[derive(Copy, Clone, Debug)]
pub struct MenuClick {
    page_index: usize,
}

impl MenuClick {
    pub fn new(page_index: usize) -> Self {
        MenuClick { page_index }
    }
}

impl Task for MenuClick {
    fn execute(&self) -> Option<Arc<dyn Event>> {
        Some(Arc::new(*self))
    }
}

impl Event for MenuClick {
    fn handle(&self, app: &mut App) {
        if let Some(page) = app.menu.page(self.page_index) {
            app.page = page;
        }
    }
}
