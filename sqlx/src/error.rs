use actix_web::http::StatusCode;
use log::error;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Sqlx(sqlx::Error),
    BlockingCanceled,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io: {}", e),
            Self::Sqlx(e) => write!(f, "Sqlx: {}", e),
            Self::BlockingCanceled => write!(f, "BlockingCanceled"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        error!("{}", self);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<actix_web::error::BlockingError<Error>> for Error {
    fn from(e: actix_web::error::BlockingError<Error>) -> Error {
        match e {
            actix_web::error::BlockingError::Error(e) => e,
            actix_web::error::BlockingError::Canceled => Error::BlockingCanceled,
        }
    }
}