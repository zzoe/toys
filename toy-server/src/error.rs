use poem::error::ResponseError;
use poem::http::StatusCode;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("系统异常")]
    InternalServerErr,
    #[error("请登录后再试")]
    UnAuthenticated,
    #[error("您无此功能权限")]
    UnAuthorized,
    #[error("数据库异常")]
    DbException(#[from] surrealdb::Error),

    /// Invalid content type.
    #[error("invalid content type `{0}`, expect: `application/octet-stream`")]
    InvalidContentType(String),

    /// `Content-Type` header is required.
    #[error("expect content type `application/octet-stream`")]
    ContentTypeRequired,

    /// Url decode error.
    #[error("parse error: {0}")]
    Parse(#[from] speedy::Error),

    #[error("注册失败")]
    SignUpFail,
    #[error("登录失败")]
    SignInFail,

    #[error("数独游戏数字异常: {0}")]
    SudokuNumInvalid(u16),
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        match self {
            Error::InternalServerErr => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SignUpFail => StatusCode::UNAUTHORIZED,
            Error::SignInFail => StatusCode::UNAUTHORIZED,
            Error::UnAuthenticated => StatusCode::UNAUTHORIZED,
            Error::UnAuthorized => StatusCode::UNAUTHORIZED,
            Error::DbException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Error::ContentTypeRequired => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Error::Parse(_) => StatusCode::BAD_REQUEST,
            Error::SudokuNumInvalid(_) => StatusCode::BAD_REQUEST,
        }
    }
}
