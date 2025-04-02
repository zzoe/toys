use crate::config::GLOBAL_CONFIG;
use crate::init::{init_log, init_trace};
use futures::FutureExt;
use rustls::crypto::ring;
use tokio::signal::ctrl_c;

mod config;
mod error;
mod init;
mod web;

#[tokio::main]
async fn main() {
    ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    let _guard = init_log();
    init_trace();
    web::start(ctrl_c().map(|_| ())).await;
}
