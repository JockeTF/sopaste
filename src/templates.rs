use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::*;

use crate::models::ListRow;
use crate::models::TextRow;
use crate::models::TreeItem;
use crate::result::Error;
use crate::storage::Pool;
use crate::syntax::Syntax;
use crate::tree::TreeRoot;

type PageResult = Result<(ContentType, String), Error>;

fn render(template: &impl Template) -> PageResult {
    Ok((ContentType::HTML, template.render()?))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[get("/")]
fn index() -> PageResult {
    render(&Index {})
}

#[derive(Template)]
#[template(path = "about.html")]
struct About {}

#[get("/about")]
fn about() -> PageResult {
    render(&About {})
}

#[derive(Template)]
#[template(path = "paste.html")]
struct Paste {
    list: ListRow,
    html: String,
}

#[get("/<id>")]
async fn paste(id: &str, pool: &Pool, syntax: &Syntax) -> PageResult {
    let list = ListRow::find(pool, id).await?;
    let text = TextRow::find(pool, id).await?;

    let lang = &list.language.decode();
    let text = &text.text.decode();
    let html = syntax.highlight(lang, text)?;

    render(&Paste { list, html })
}

#[get("/<id>/raw")]
async fn raw(id: &str, pool: &Pool) -> PageResult {
    let row = TextRow::find(pool, id).await?;
    let text = row.text.decode().into_owned();

    Ok((ContentType::Text, text))
}

#[get("/<id>/tree")]
async fn tree(id: &str, pool: &Pool) -> PageResult {
    let items = TreeItem::list(pool, id).await?;

    if items.is_empty() {
        return Err(Error::Status(Status::NotFound));
    }

    render(&TreeRoot::new(&items))
}

pub fn routes() -> Vec<Route> {
    routes![index, about, paste, raw, tree]
}
