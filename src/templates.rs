use std::result::Result;

use askama::Template;
use sea_orm::EntityTrait;

use rocket::http::ContentType;
use rocket::*;

use crate::entities::list;
use crate::entities::text;
use crate::result::Error;
use crate::storage::Pool;
use crate::syntax::Syntax;

type PageResult = Result<(ContentType, String), Error>;

fn render(template: impl Template) -> PageResult {
    Ok((ContentType::HTML, template.render()?))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[get("/")]
fn index() -> PageResult {
    render(Index {})
}

#[derive(Template)]
#[template(path = "about.html")]
struct About {}

#[get("/about")]
fn about() -> PageResult {
    render(About {})
}

#[derive(Template)]
#[template(path = "paste.html")]
struct Paste {
    list: list::Model,
    html: String,
}

#[get("/<id>")]
async fn paste(id: String, pool: &Pool, syntax: &Syntax) -> PageResult {
    let list = list::Entity::find_by_id(id.clone())
        .one(pool.conn())
        .await?
        .unwrap();

    let text = text::Entity::find_by_id(id.clone())
        .one(pool.conn())
        .await?
        .unwrap();

    let text = &text.text;
    let lang = &list.language;
    let html = syntax.highlight(lang, text)?;

    render(Paste { list, html })
}

#[get("/<id>/raw")]
async fn raw(id: String, pool: &Pool) -> PageResult {
    let row = text::Entity::find_by_id(id.clone())
        .one(pool.conn())
        .await?
        .unwrap();

    Ok((ContentType::Text, row.text))
}

// #[get("/<id>/tree")]
// async fn tree(id: &str, pool: &Pool) -> PageResult {
//     let items = TreeItem::list(pool, id).await?;
//
//     if items.is_empty() {
//         return Err(Error::Status(Status::NotFound));
//     }
//
//     render(TreeRoot::new(&items))
// }

pub fn routes() -> Vec<Route> {
    routes![index, about, paste, raw]
}
