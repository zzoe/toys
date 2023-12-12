use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("transparent")]
    HttpError(#[from] reqwest::Error),
    #[error("响应失败: {status}-{msg}")]
    ResponseError { status: StatusCode, msg: String },
    #[error("系统异常")]
    ServerException,
}
