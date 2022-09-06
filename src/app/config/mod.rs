use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::app::config::cal::CalConfig;

pub mod cal;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ToyConfig {
    pub cal_cfg: Rc<RefCell<CalConfig>>,
}
