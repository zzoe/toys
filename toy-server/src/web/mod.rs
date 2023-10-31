use std::future::Future;
use std::time::Duration;

use arc_swap::access::Access;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::{CatchPanic, Compression, NormalizePath, Tracing, TrailingSlash};
use poem::session::{CookieConfig, ServerSession};
use poem::{post, EndpointExt, IntoEndpoint, Route, Server};

use crate::config::Config;
use crate::web::auth::{sign_in, sign_up, Auth};
use crate::web::session::SurrealStorage;
use crate::GLOBAL_CONFIG;

pub(crate) mod auth;
pub(crate) mod database;
pub(crate) mod session;

pub(crate) async fn start(signal: impl Future<Output = ()>) {
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &Config| &cfg.web)
        .load();

    let route = Route::new()
        .at("/sign_up", post(sign_up))
        .at("/sign_in", post(sign_in))
        .nest("/api", post(apis()))
        .nest(
            "/",
            StaticFilesEndpoint::new(&cfg.assets_path)
                .index_file("index.html")
                .fallback_to_index(),
        )
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(Compression::new())
        .with(Tracing)
        .with(CatchPanic::new())
        .with(ServerSession::new(
            CookieConfig::default(),
            SurrealStorage::new().await.unwrap(),
        ));

    let res = Server::new(TcpListener::bind(&cfg.address))
        .run_with_graceful_shutdown(route, signal, Some(Duration::from_secs(10)))
        .await;

    if let Err(e) = res {
        tracing::error!("服务异常: {}", e);
    }
}

fn apis() -> impl IntoEndpoint {
    Route::new().with(Auth {})
}
