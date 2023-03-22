use askama::Template;

use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(Debug)]
pub enum Error {
    Askama(askama::Error),
    Sqlx(sqlx::Error),
    Status(StatusCode),
    Syntect(syntect::Error),
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage(StatusCode);

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

impl From<StatusCode> for Error {
    fn from(s: StatusCode) -> Self {
        Error::Status(s)
    }
}

impl From<syntect::Error> for Error {
    fn from(e: syntect::Error) -> Self {
        Error::Syntect(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::Sqlx(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
            Error::Status(status) => status,

            error => {
                eprintln!("{error:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        match ErrorPage(status).render() {
            Ok(page) => (status, Html::from(page)).into_response(),
            Err(_) => (status, status.to_string()).into_response(),
        }
    }
}
