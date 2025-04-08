use std::fs;
use std::future::Future;
use std::time::Duration;

use arc_swap::access::Access;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener};
use poem::middleware::{CatchPanic, Compression, NormalizePath, Tracing, TrailingSlash};
use poem::session::{CookieConfig, ServerSession};
use poem::{handler, post, EndpointExt, IntoEndpoint, Route, Server};

use crate::config;
use crate::web::auth::{logout, sign_check, sign_in, sign_up, Auth};
use crate::web::content_type_utf8_mw::ContentTypeUtf8;
use crate::web::session::SurrealStorage;
use crate::GLOBAL_CONFIG;

pub(crate) mod auth;
mod content_type_utf8_mw;
pub(crate) mod database;
pub(crate) mod session;
pub(crate) mod speedy_data;
mod sudoku;

pub(crate) async fn start(signal: impl Future<Output = ()>) {
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &config::Config| &cfg.web)
        .load();

    let route = Route::new()
        .nest("/api", apis().await)
        .nest(
            "/",
            StaticFilesEndpoint::new(&cfg.assets_path)
                .prefer_utf8(true)
                .index_file("index.html")
                .fallback_to_index()
                .with(ContentTypeUtf8),
        )
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(Compression::new())
        .with(Tracing)
        .with(CatchPanic::new());

    let res = Server::new(
        TcpListener::bind(&cfg.address).rustls(
            RustlsConfig::new().fallback(
                RustlsCertificate::new()
                    .key(fs::read("localhost+3-key.pem").unwrap())
                    .cert(fs::read("localhost+3.pem").unwrap()),
            ),
        ),
    )
    .run_with_graceful_shutdown(route, signal, Some(Duration::from_secs(30)))
    .await;

    if let Err(e) = res {
        log::error!("服务异常: {}", e);
    }
}

async fn apis() -> impl IntoEndpoint {
    Route::new()
        .at("/sign_up", post(sign_up))
        .at("/sign_in", post(sign_in))
        .at("/sign_check", post(sign_check))
        .nest("/", need_auth())
        .with(ServerSession::new(
            CookieConfig::default(),
            SurrealStorage::new().await.expect("Session数据库异常"),
        ))
}
fn need_auth() -> impl IntoEndpoint {
    Route::new()
        .at("/logout", post(logout))
        .at("/reload", post(reload))
        .at("/sudoku", post(sudoku::resolve))
        .with(Auth {})
}

#[handler]
async fn reload() {
    config::reload();
}
