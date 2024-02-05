use poem::http::{header, HeaderValue};
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};

#[derive(Default)]
pub struct ContentTypeUtf8;

impl<E: Endpoint> Middleware<E> for ContentTypeUtf8 {
    type Output = CTUtf8Endpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CTUtf8Endpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct CTUtf8Endpoint<E> {
    inner: E,
}

#[async_trait::async_trait]
impl<E: Endpoint> Endpoint for CTUtf8Endpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let cache_control = cache_control(req.uri().path());

        match self.inner.call(req).await {
            Ok(res) => {
                let mut res = res.into_response();

                res.headers_mut().insert(
                    "X-Content-Type-Options",
                    HeaderValue::from_str("nosniff").unwrap(),
                );
                res.headers_mut().insert(
                    "Cache-Control",
                    HeaderValue::from_str(cache_control).unwrap(),
                );

                if let Some(ct) = res.header(header::CONTENT_TYPE) {
                    if mime::IMAGE_SVG.eq(&ct) {
                        let res = res.set_content_type("image/svg+xml; charset=utf-8");
                        return Ok(res);
                    }
                };
                Ok(res)
            }
            Err(err) => Err(err),
        }
    }
}

fn cache_control<'a>(path: &str) -> &'a str {
    match path {
        // "/" | "/tailwind.css" | "web_bg.wasm" => "no-cache",
        "/" | "/tailwind.css" | "web_bg.wasm" => "max-age=180, immutable",
        _ => "max-age=31536000, immutable",
    }
}
