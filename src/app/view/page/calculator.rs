use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_channel::Sender;
use eframe::egui;
use eframe::egui::Ui;
use eframe::egui::{Align, Color32, ComboBox, Layout, RichText, TextEdit, Widget};
use egui_extras::{Size, TableBuilder};
use rayon::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy::ToZero;

use crate::app::config::cal::{CalConfig, Product, RenewType, TermType};
use crate::app::view::View;
use crate::app::worker::calculator::{
    check_date, CalEvent, TentativeCalculation, TentativeInterest,
};
use crate::app::worker::Task;

#[derive(Debug)]
pub struct Calculator {
    sender: Sender<Arc<dyn Task>>,
    warn: Result<()>,
    cache: HashMap<TentativeCalculation, TentativeInterest>,
    cal_cfg: Rc<RefCell<CalConfig>>,
}

impl Calculator {
    pub fn new(sender: Sender<Arc<dyn Task>>, cal_cfg: Rc<RefCell<CalConfig>>) -> Self {
        Self {
            sender,
            warn: Ok(()),
            cache: Default::default(),
            cal_cfg,
        }
    }

    pub fn refresh_cache(&mut self, cal_event: &CalEvent) {
        self.cache.insert(cal_event.req, cal_event.res);

        let mut cfg = self.cal_cfg.borrow_mut();
        let CalConfig { order, products } = cfg.deref_mut();

        products.iter_mut().for_each(|product| {
            if let Some(res) = self.cache.get(&TentativeCalculation::new(order, product)) {
                product.interest = res.interest;
                product.bean_int = res.bean_int;
            }
        })
    }

    fn calc(&mut self, index: Option<usize>) {
        let mut cfg = self.cal_cfg.borrow_mut();
        let CalConfig { order, products } = cfg.deref_mut();

        self.warn = check_date(order);

        if self.warn.is_ok() {
            products
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, product)| {
                    if index.is_some() && index.ne(&Some(i)) {
                        return;
                    }

                    let req = Arc::new(TentativeCalculation::new(order, product));
                    if let Some(res) = self.cache.get(&req) {
                        product.interest = res.interest;
                        product.bean_int = res.bean_int;
                    } else {
                        self.sender.send_blocking(req).unwrap();
                    }
                });
        }
    }

    fn principal_changed(&mut self, principal: &str) {
        if let Ok(mut v) = principal.parse::<Decimal>() {
            v = v.round_dp_with_strategy(2, ToZero);
            if v >= Decimal::new(1000_0000_0000, 0) {
                self.warn = Err(anyhow!("一千亿啊，土豪，还需要算吗？"))
            } else {
                self.cal_cfg.borrow_mut().order.principal = v;
                self.calc(None);
            }
        }
    }

    fn save_date_changed(&mut self, save_date: &str) {
        if let Ok(v) = save_date.parse() {
            self.cal_cfg.borrow_mut().order.save_date = v;
            self.calc(None);
        }
    }

    fn draw_date_changed(&mut self, draw_date: &str) {
        if let Ok(v) = draw_date.parse() {
            self.cal_cfg.borrow_mut().order.draw_date = v;
            self.calc(None);
        }
    }

    fn term_changed(&mut self, term: &str, row_index: usize) {
        if let Ok(v) = term.parse() {
            self.cal_cfg.borrow_mut().products[row_index].term = v;
            self.calc(Some(row_index));
        }
    }

    fn term_type_changed(&mut self, term_type: usize, row_index: usize) {
        self.cal_cfg.borrow_mut().products[row_index].term_type = TermType::from(term_type);
        self.calc(Some(row_index));
    }

    fn int_rate_changed(&mut self, int_rate: &str, row_index: usize) {
        if let Ok(mut v) = int_rate.parse::<Decimal>() {
            v = v.round_dp_with_strategy(2, ToZero);
            if v > Decimal::TEN {
                self.warn = Err(anyhow!("哪里有这么高的利率，苟富贵勿相忘啊，兄弟！"));
            } else {
                self.cal_cfg.borrow_mut().products[row_index].int_rate = v;
                self.calc(Some(row_index));
            }
        }
    }

    fn bean_rate_changed(&mut self, bean_rate: &str, row_index: usize) {
        if let Ok(mut v) = bean_rate.parse::<Decimal>() {
            v = v.round_dp_with_strategy(2, ToZero);
            if v > Decimal::TEN {
                self.warn = Err(anyhow!("哪里有这么高的利率，苟富贵勿相忘啊，兄弟！"));
            } else {
                self.cal_cfg.borrow_mut().products[row_index].bean_rate = v;
                self.calc(Some(row_index));
            }
        }
    }

    fn renew_type_changed(&mut self, renew_type: usize, row_index: usize) {
        self.cal_cfg.borrow_mut().products[row_index].renew_type = RenewType::from(renew_type);
        self.calc(Some(row_index));
    }
}

impl View for Calculator {
    fn name(&self) -> &str {
        "🖩 计算器"
    }

    fn view(&mut self, ui: &mut Ui) {
        if let Err(e) = &self.warn {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                let warn = RichText::from(e.to_string()).color(Color32::RED);
                ui.label(warn);
            });
        }

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size * 2.0;

        TableBuilder::new(ui)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Size::remainder())
            .column(Size::remainder())
            .column(Size::remainder())
            .column(Size::remainder())
            .header(text_height, |mut header| {
                header.col(|ui| {
                    ui.heading("本金");
                    let mut principal = format!("{:.2}", self.cal_cfg.borrow().order.principal);
                    if ui.text_edit_singleline(&mut principal).changed() {
                        self.principal_changed(&*principal);
                    };
                });

                header.col(|ui| {
                    ui.heading("购买日期：");
                    let mut save_date = self.cal_cfg.borrow().order.save_date.to_string();
                    if ui.text_edit_singleline(&mut save_date).changed() {
                        save_date.truncate(8);
                        self.save_date_changed(&*save_date);
                    }
                });

                header.col(|ui| {
                    ui.heading("支取日期：");
                    let mut draw_date = self.cal_cfg.borrow().order.draw_date.to_string();
                    if ui.text_edit_singleline(&mut draw_date).changed() {
                        draw_date.truncate(8);
                        self.draw_date_changed(&*draw_date);
                    }
                });

                header.col(|ui| {
                    ui.heading(format!("天数：{}", self.cal_cfg.borrow().order.days));
                });
            });

        ui.separator();

        let total_rows = self.cal_cfg.borrow().products.len();
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Size::initial(100.0))
            .column(Size::remainder())
            .column(Size::remainder())
            .column(Size::initial(150.0))
            .column(Size::remainder())
            .column(Size::remainder())
            .column(Size::initial(50.0))
            .header(text_height, |mut header| {
                header.col(|ui| {
                    ui.heading("存期");
                });
                header.col(|ui| {
                    ui.heading("利率(%)");
                });
                header.col(|ui| {
                    ui.heading("邦豆利率(%)");
                });
                header.col(|ui| {
                    ui.heading("续存类型");
                });
                header.col(|ui| {
                    ui.heading("利息");
                });
                header.col(|ui| {
                    ui.heading("邦豆利息");
                });
                header.col(|ui| {
                    if ui.button("添加").clicked() {
                        self.cal_cfg.borrow_mut().products.push(Product::default());
                    }
                });
            })
            .body(|body| {
                body.rows(text_height, total_rows, |row_index, mut row| {
                    if self.cal_cfg.borrow().products.len() <= row_index {
                        return;
                    }
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            let mut term =
                                self.cal_cfg.borrow().products[row_index].term.to_string();
                            if TextEdit::singleline(&mut term)
                                .desired_width(20.0)
                                .ui(ui)
                                .changed()
                            {
                                self.term_changed(&*term, row_index);
                            }

                            let mut term_type =
                                self.cal_cfg.borrow().products[row_index].term_type as usize;
                            if ComboBox::from_id_source(format!("存期类型{}", row_index))
                                .width(20.0)
                                .show_index(ui, &mut term_type, 3, |i| {
                                    TermType::from(i).to_string()
                                })
                                .changed()
                            {
                                self.term_type_changed(term_type, row_index);
                            };
                        });
                    });
                    row.col(|ui| {
                        let mut int_rate =
                            format!("{:.2}", self.cal_cfg.borrow().products[row_index].int_rate);
                        if ui.text_edit_singleline(&mut int_rate).changed() {
                            self.int_rate_changed(&*int_rate, row_index);
                        };
                    });
                    row.col(|ui| {
                        let mut bean_rate =
                            format!("{:.2}", self.cal_cfg.borrow().products[row_index].bean_rate);
                        if ui.text_edit_singleline(&mut bean_rate).changed() {
                            self.bean_rate_changed(&*bean_rate, row_index);
                        };
                    });
                    row.col(|ui| {
                        let mut renew_type =
                            self.cal_cfg.borrow().products[row_index].renew_type as usize;
                        if ComboBox::from_id_source(format!("续存类型{}", row_index))
                            .width(140.0)
                            .show_index(ui, &mut renew_type, 3, |i| RenewType::from(i).to_string())
                            .changed()
                        {
                            self.renew_type_changed(renew_type, row_index);
                        };
                    });
                    row.col(|ui| {
                        ui.label(format!(
                            "{:.2}",
                            self.cal_cfg.borrow().products[row_index].interest
                        ));
                    });
                    row.col(|ui| {
                        ui.label(format!(
                            "{:.2}",
                            self.cal_cfg.borrow().products[row_index].bean_int
                        ));
                    });
                    row.col(|ui| {
                        if ui.button("删除").clicked() {
                            self.cal_cfg.borrow_mut().products.remove(row_index);
                        }
                    });
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
