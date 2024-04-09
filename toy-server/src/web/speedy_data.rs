use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use poem::http::{header, StatusCode};
use poem::{FromRequest, IntoResponse, Request, RequestBody, Response, Result};
use speedy::{Endianness, Readable, Writable};
use tracing::info;

use crate::error::Error;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Speedy<T>(pub T);

impl<T> Deref for Speedy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Speedy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: Debug + Readable<'a, Endianness>> FromRequest<'a> for Speedy<T> {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        let content_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|content_type| content_type.to_str().ok())
            .ok_or(Error::ContentTypeRequired)?;
        if !is_octet_stream(content_type) {
            return Err(Error::InvalidContentType(content_type.into()).into());
        }
        let body_data = body.take()?.into_bytes().await?;
        let s = T::read_from_buffer_copying_data_with_ctx(Endianness::LittleEndian, &body_data)
            .map_err(Error::Parse)?;
        info!("Body解析成功：{s:?}");

        Ok(Self(s))
    }
}

fn is_octet_stream(content_type: &str) -> bool {
    matches!(content_type.parse::<mime::Mime>(), 
        Ok(content_type) if content_type.type_() == "application" 
        && (content_type.subtype() == "octet-stream"
        || content_type
            .suffix()
            .map_or(false, |v| v == "octet-stream")))
}

impl<T: Writable<Endianness> + Send> IntoResponse for Speedy<T> {
    fn into_response(self) -> Response {
        let data = match self.0.write_to_vec_with_ctx(Endianness::LittleEndian) {
            Ok(data) => data,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string())
            }
        };
        Response::builder()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(data)
    }
}
