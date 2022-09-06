use std::cmp::min;
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy::{MidpointAwayFromZero, ToZero};
use time::{util, Date, Duration, Month};

use crate::app::config::cal::{Order, Product, RenewType, TermType};
use crate::app::worker::{Event, Task};
use crate::app::App;

//req 需要计算的key (本金-购买日期-支取日期-产品存期-存期类型-利率-邦豆利率-滚存类型)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TentativeCalculation {
    pub principal: Decimal,
    pub save_date: u32,
    pub draw_date: u32,
    pub term: u8,
    pub term_type: TermType,
    pub int_rate: Decimal,
    pub bean_rate: Decimal,
    pub renew_type: RenewType,
}

impl TentativeCalculation {
    pub fn new(order: &Order, product: &Product) -> Self {
        Self {
            principal: order.principal,
            save_date: order.save_date,
            draw_date: order.draw_date,
            term: product.term,
            term_type: product.term_type,
            int_rate: product.int_rate,
            bean_rate: product.bean_rate,
            renew_type: product.renew_type,
        }
    }
}

//res 计算结果HashMap<key,value> ()
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TentativeInterest {
    pub interest: Decimal,
    pub bean_int: Decimal,
}

pub fn u32_to_date(date: u32) -> Result<Date> {
    Date::from_calendar_date(
        (date / 10000) as i32,
        Month::try_from((date / 100 % 100) as u8).map_err(|e| anyhow!("月份有误！{e}"))?,
        (date % 100) as u8,
    )
    .map_err(|e| anyhow!("日期有误!{e}"))
}

pub fn check_date(order: &mut Order) -> Result<()> {
    if order.save_date < 10000101
        || order.save_date > 99991231
        || order.draw_date < 10000101
        || order.draw_date > 99991231
        || order.save_date > order.draw_date
    {
        bail!("穿越时空？")
    }

    let save_date = u32_to_date(order.save_date)?;
    let draw_date = u32_to_date(order.draw_date)?;

    order.days = (draw_date.to_julian_day() - save_date.to_julian_day()) as i32;
    if order.days > 36500 {
        bail!("你确定可以存一个世纪？")
    }

    Ok(())
}

impl Task for TentativeCalculation {
    fn execute(&self) -> Option<Arc<dyn Event>> {
        if self.term < 1 {
            return None;
        }

        let save_date = u32_to_date(self.save_date).unwrap();
        let draw_date = u32_to_date(self.draw_date).unwrap();

        let mut start_date = save_date;
        let mut principal = self.principal;
        let mut interest = Decimal::ZERO;
        let mut bean_int = Decimal::ZERO;
        let mut int_rate = self.int_rate;
        let mut bean_rate = self.bean_rate;

        while start_date < draw_date {
            let mut end_date = match &self.term_type {
                TermType::D => start_date.saturating_add(Duration::days(self.term as i64)),
                TermType::M => {
                    let month = start_date.month() as u8 + self.term - 1;
                    let year = start_date.year() + month as i32 / 12;
                    let month = Month::try_from(month % 12 + 1).unwrap();
                    let max_day = util::days_in_year_month(year, month);

                    Date::from_calendar_date(year, month, min(start_date.day(), max_day)).unwrap()
                }
                TermType::Y => {
                    let year = start_date.year() + self.term as i32;
                    let month = start_date.month();
                    let max_day = util::days_in_year_month(year, month);
                    Date::from_calendar_date(year, month, min(start_date.day(), max_day)).unwrap()
                }
            };

            if end_date > draw_date {
                end_date = draw_date;
                int_rate = Decimal::new(35, 2);
                bean_rate = Decimal::ZERO;
            }

            let days = Decimal::new(
                (end_date.to_julian_day() - start_date.to_julian_day()) as i64,
                0,
            );

            // 利息2位小数四舍五入, 溢出归0
            interest = calc_interest(principal, int_rate, days)
                .and_then(|d| d.checked_add(interest))
                .map(|d| d.round_dp_with_strategy(2, MidpointAwayFromZero))
                .unwrap_or_default();

            // 邦豆2位小数之后全部舍弃, 溢出归0
            bean_int = calc_interest(principal, bean_rate, days)
                .and_then(|d| d.checked_add(bean_int))
                .map(|d| d.round_dp_with_strategy(2, ToZero))
                .unwrap_or_default();

            match self.renew_type {
                RenewType::N => {
                    break;
                }
                RenewType::P => {}
                RenewType::I => {
                    principal = principal.checked_add(interest).unwrap_or_default();
                    interest = Decimal::ZERO;
                }
            }
            start_date = end_date;
        }

        Some(Arc::new(CalEvent {
            req: *self,
            res: TentativeInterest {
                interest: principal - self.principal + interest,
                bean_int,
            },
        }))
    }
}

fn calc_interest(principal: Decimal, rate: Decimal, days: Decimal) -> Option<Decimal> {
    days.checked_div(Decimal::new(360, 0))
        .and_then(|d| d.checked_mul(rate))
        .and_then(|d| d.checked_div(Decimal::ONE_HUNDRED))
        .and_then(|d| d.checked_mul(principal))
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct CalEvent {
    pub req: TentativeCalculation,
    pub res: TentativeInterest,
}

impl Event for CalEvent {
    fn handle(&self, app: &mut App) {
        app.calculator.borrow_mut().refresh_cache(self);
    }
}
