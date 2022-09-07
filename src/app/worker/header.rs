use std::sync::Arc;

use crate::app::worker::{Event, Task};
use crate::app::App;

#[derive(Copy, Clone, Debug)]
pub struct LogoClick {}

impl Task for LogoClick {
    fn execute(&self) -> Option<Arc<dyn Event>> {
        Some(Arc::new(*self))
    }
}

impl Event for LogoClick {
    fn handle(&self, app: &mut App) {
        app.menu_switch = !app.menu_switch;
    }
}
