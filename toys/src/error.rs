use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("transparent")]
    HttpError(#[from] reqwest::Error),
    #[error("speedy序列化失败: {0}")]
    ParseError(#[from] speedy::Error),
    #[error("响应失败: {status}")]
    ResponseError { status: StatusCode },
    #[error("系统异常")]
    ServerException,
}
