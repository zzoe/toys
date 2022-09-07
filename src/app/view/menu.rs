use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use async_channel::Sender;
use eframe::egui;
use eframe::egui::{Label, Sense, Ui};

use crate::app::config::ToyConfig;
use crate::app::view::page::calculator::Calculator;
use crate::app::view::page::font_book::FontBook;
use crate::app::view::page::home::Home;
use crate::app::view::View;
use crate::app::worker::menu::MenuClick;
use crate::app::worker::Task;

macro_rules! add_label {
    ($ui:tt, $sender:expr, $page:tt, $page_index:tt) => {
        let label = Label::new($page.borrow().name()).sense(Sense::click());
        if $ui.add(label).clicked() {
            $sender
                .send_blocking(Arc::new(MenuClick::new($page_index)))
                .ok();
        }
    };
}

#[derive(Debug)]
pub struct Menu {
    sender: Sender<Arc<dyn Task>>,
    search: String,
    menus: Vec<Rc<RefCell<dyn View>>>,
}

impl Menu {
    pub fn new(sender: Sender<Arc<dyn Task>>, cfg: &ToyConfig) -> Self {
        let calculator = Rc::new(RefCell::new(Calculator::new(
            sender.clone(),
            Rc::clone(&cfg.cal_cfg),
        )));
        Menu {
            search: String::new(),
            menus: vec![
                Rc::new(RefCell::new(Home {})),
                Rc::new(RefCell::new(FontBook::default())),
                calculator,
            ],
            sender,
        }
    }

    // pub fn push(&mut self, page: Rc<RefCell<dyn View>>) {
    //     self.menus.push(page)
    // }

    pub fn home(&self) -> Rc<RefCell<dyn View>> {
        Rc::clone(&self.menus[0])
    }

    pub fn page(&self, page_index: usize) -> Option<Rc<RefCell<dyn View>>> {
        self.menus.get(page_index).map(Rc::clone)
    }
}

impl View for Menu {
    fn name(&self) -> &str {
        "menu"
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
            ui.spacing_mut().indent_ends_with_horizontal_line = true;
            ui.add_space(10.0);

            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("搜索"));

            self.menus
                .iter()
                .enumerate()
                .filter(|(page_index, page)| {
                    if self.search.is_empty() || *page_index == 0 {
                        true
                    } else {
                        page.borrow()
                            .name()
                            .to_ascii_lowercase()
                            .contains(&self.search.to_ascii_lowercase())
                    }
                })
                .for_each(|(page_index, page)| {
                    add_label!(ui, self.sender, page, page_index);

                    if page_index == 0 {
                        ui.separator();
                    }
                });
        });
    }

    fn any(&self) -> &dyn Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
