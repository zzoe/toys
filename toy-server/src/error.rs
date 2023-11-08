use poem::http::StatusCode;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("系统异常")]
    InternalServerErr,
    #[error("请勿重复操作")]
    DoNotRepeat,
    #[error("注册失败")]
    SignUpFail,
    #[error("登录失败")]
    SignInFail,
    #[error("请登录后再试")]
    UnAuthenticated,
    #[error("您无此功能权限")]
    UnAuthorized,
}

pub trait ErrorConv<T> {
    fn poem_error(self, err: Error, status_code: StatusCode) -> Result<T, poem::Error>;
    fn bad_request(self, err: Error) -> Result<T, poem::Error>;
    fn unauthorized(self) -> Result<T, poem::Error>;
    fn internal_server_error(self) -> Result<T, poem::Error>;
}

impl<T, E: std::error::Error> ErrorConv<T> for Result<T, E> {
    fn poem_error(self, err: Error, status_code: StatusCode) -> Result<T, poem::Error> {
        self.map_err(|e| {
            error!("{e}");
            poem::Error::new(err, status_code)
        })
    }

    fn bad_request(self, err: Error) -> Result<T, poem::Error> {
        self.map_err(|e| {
            error!("{e}");
            poem::error::BadRequest(err)
        })
    }

    fn unauthorized(self) -> Result<T, poem::Error> {
        self.map_err(|e| {
            error!("{e}");
            poem::error::Unauthorized(Error::UnAuthorized)
        })
    }

    fn internal_server_error(self) -> Result<T, poem::Error> {
        self.map_err(|e| {
            error!("{e}");
            poem::error::InternalServerError(Error::InternalServerErr)
        })
    }
}
