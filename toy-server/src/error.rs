use poem::http::StatusCode;

pub const SIGN_UP_ERROR: &str = "注册失败";
pub const SIGN_IN_ERROR: &str = "登录失败";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] surrealdb::Error),
    #[error("会话异常")]
    SessionNotFound,
}

impl Error {
    pub fn to_poem_error(self, status_code: StatusCode, msg: impl Into<String>) -> poem::Error {
        let mut err = poem::Error::new(self, status_code);
        err.set_error_message(msg);
        err
    }
}

impl From<Error> for poem::Error {
    fn from(err: Error) -> Self {
        poem::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn surreal_to_poem(err: surrealdb::Error) -> poem::Error {
    Error::from(err).into()
}
