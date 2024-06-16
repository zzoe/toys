use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("transparent")]
    Http(#[from] reqwest::Error),
    #[error("speedy序列化失败: {0}")]
    Parse(#[from] speedy::Error),
    #[error("响应失败: {status}")]
    Response { status: StatusCode },
    // #[error("系统异常")]
    // ServerException,
}
