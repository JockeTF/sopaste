use rocket::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Result;
use rocket::Request;

#[derive(Debug)]
pub enum Error {
    Askama(askama::Error),
    Sqlx(sqlx::Error),
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

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        use Error::*;

        let status = match self {
            Sqlx(sqlx::Error::RowNotFound) => Status::NotFound,

            error => {
                error!("{:?}", error);
                Status::InternalServerError
            }
        };

        Err(status)
    }
}
