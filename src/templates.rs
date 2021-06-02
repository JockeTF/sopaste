use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::response::content::Custom;
use rocket::*;

use crate::models::ListRow;
use crate::models::TextRow;
use crate::result::Error;
use crate::storage::Pool;
use crate::syntax::Syntax;

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
    name: &'a str,
    desc: &'a str,
    date: &'a str,
    time: &'a str,
    text: &'a str,
}

#[get("/<id>")]
async fn paste(id: &str, pool: &Pool, syntax: &Syntax) -> PageResult {
    let list = ListRow::find(pool, id).await?;
    let text = TextRow::find(pool, id).await?;

    let lang = &list.language.unwrap_or_else(String::new);
    let text = &text.text.unwrap_or_else(String::new);
    let highlighted = syntax.highlight(lang, text);

    let paste = Paste {
        name: &list.name.unwrap_or_else(|| "Anonymous".into()),
        desc: &list.description.unwrap_or_else(String::new),
        date: &list.date.to_string(),
        time: &list.time.to_string(),
        text: &highlighted,
    };

    render(paste)
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
