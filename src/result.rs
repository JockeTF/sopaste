use rocket::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Result;
use rocket::Request;

#[derive(Debug)]
pub enum Error {
    Askama(askama::Error),
    Sqlx(sqlx::Error),
    Status(Status),
    Syntect(syntect::Error),
}

impl From<askama::Error> for Error {
    fn from(e: askama::Error) -> Self {
        Error::Askama(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sqlx(e)
    }
}

impl From<Status> for Error {
    fn from(s: Status) -> Self {
        Error::Status(s)
    }
}

impl From<syntect::Error> for Error {
    fn from(e: syntect::Error) -> Self {
        Error::Syntect(e)
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        Err(match self {
            Error::Sqlx(sqlx::Error::RowNotFound) => Status::NotFound,
            Error::Status(status) => status,

            error => {
                error!("{:?}", error);
                Status::InternalServerError
            }
        })
    }
}
