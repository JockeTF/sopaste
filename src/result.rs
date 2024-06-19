use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use derive_more::From;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    Askama(askama::Error),
    Sqlx(sqlx::Error),
    Status(StatusCode),
    Syntect(syntect::Error),
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage(StatusCode);

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

pub async fn fallback() -> Error {
    Error::Status(StatusCode::NOT_FOUND)
}
