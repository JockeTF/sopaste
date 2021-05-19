use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::response::content::Custom;
use rocket::*;

use sqlx::query_as;

use crate::models::TextRow;
use crate::result::Error;
use crate::storage::Pool;

type PageResult = Result<Custom<String>, Error>;

fn render(template: impl Template) -> PageResult {
    Ok(Custom(ContentType::HTML, template.render()?))
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

    let query = query_as(sql).bind(id);
    let row: TextRow = query.fetch_one(&**pool).await?;
    let text = row.text.unwrap_or_else(String::new);

    Ok(Custom(ContentType::Text, text))
}

pub fn routes() -> Vec<Route> {
    routes![index, raw]
}
