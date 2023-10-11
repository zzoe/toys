use std::future::Future;
use std::time::Duration;

use arc_swap::access::Access;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::{CatchPanic, Compression, NormalizePath, Tracing, TrailingSlash};
use poem::{EndpointExt, Route, Server};

use crate::config::Config;
use crate::GLOBAL_CONFIG;

pub(crate) async fn start(signal: impl Future<Output = ()>) {
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &Config| &cfg.web)
        .load();

    let route = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new(&cfg.assets_path)
                .index_file("index.html")
                .fallback_to_index(),
        )
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(Compression::new())
        .with(Tracing)
        .with(CatchPanic::new());

    let res = Server::new(TcpListener::bind(&cfg.address))
        .run_with_graceful_shutdown(route, signal, Some(Duration::from_secs(10)))
        .await;

    if let Err(e) = res {
        tracing::error!("服务异常: {}", e);
    }
}
