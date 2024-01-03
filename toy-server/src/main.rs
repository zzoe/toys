use arc_swap::access::Access;
use async_std::channel::bounded;
use async_std::task;
use time::macros::format_description;
use time::UtcOffset;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::config::{Config, GLOBAL_CONFIG};

mod config;
pub(crate) mod error;
mod web;

fn main() {
    // 初始化日志
    let _guard = init_log();

    // ctrl-c
    let (s, r) = bounded::<()>(1);
    ctrlc::set_handler(move || {
        s.close();
    })
    .unwrap();

    task::block_on(web::start(async {
        r.recv().await.ok();
    }))
}

fn init_log() -> WorkerGuard {
    //加载配置
    config::reload();
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &Config| &cfg.log)
        .load();

    let file_appender = tracing_appender::rolling::daily(&*cfg.directory, &*cfg.file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_thread_ids(true)
        .with_max_level(cfg.level.parse::<Level>().expect("日志级别配置错误"))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            time_format,
        ))
        .with_writer(non_blocking)
        .init();

    guard
}
