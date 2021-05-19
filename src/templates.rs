use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::content::Custom;
use rocket::*;

use sqlx::query_as;

use crate::models::TextRow;
use crate::storage::Pool;

type PageResult = Result<Custom<String>, Status>;

fn render(template: impl Template) -> PageResult {
    match template.render() {
        Ok(result) => Ok(Custom(ContentType::HTML, result)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[get("/")]
fn index() -> PageResult {
    render(Index {})
}

#[get("/<id>/raw")]
async fn raw(id: &str, pool: &Pool) -> PageResult {
    let sql = "
        SELECT
            text.*
        FROM
            list
            INNER JOIN text ON list.id = text.id
        WHERE
            removed IS FALSE
            AND list.id = ?
    ";

    let result = query_as(sql).bind(id).fetch_optional(&**pool).await;

    let row: TextRow = result
        .map_err(|_| Status::InternalServerError)?
        .ok_or_else(|| Status::NotFound)?;

    let text = row.text.unwrap_or_else(String::new);

    Ok(Custom(ContentType::Text, text))
}

pub fn routes() -> Vec<Route> {
    routes![index, raw]
}
