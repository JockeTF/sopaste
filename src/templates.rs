use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::response::content::Custom;
use rocket::*;

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

#[derive(Template)]
#[template(path = "paste.html")]
struct Paste<'a> {
    text: &'a str,
}

#[get("/<id>")]
async fn paste(id: &str, pool: &Pool) -> PageResult {
    let row = TextRow::find(pool, id).await?;
    let text = row.text.unwrap_or_else(String::new);

    render(Paste { text: &text })
}

#[get("/<id>/raw")]
async fn raw(id: &str, pool: &Pool) -> PageResult {
    let row = TextRow::find(pool, id).await?;
    let text = row.text.unwrap_or_else(String::new);

    Ok(Custom(ContentType::Text, text))
}

pub fn routes() -> Vec<Route> {
    routes![index, paste, raw]
}
