use std::result::Result;

use askama::Template;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::menu::PasteMenu;
use crate::menu::PastePage;
use crate::models::ListRow;
use crate::models::TextRow;
use crate::models::TreeItem;
use crate::result::Error;
use crate::state::AppState;
use crate::state::Pool;
use crate::state::Syntax;
use crate::tree::TreeRoot;

type PageResult<T> = Result<(StatusCode, T), Error>;
type HtmlResult = PageResult<Html<String>>;
type TextResult = PageResult<String>;

fn render(template: &impl Template) -> HtmlResult {
    Ok((StatusCode::OK, Html::from(template.render()?)))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

async fn index() -> HtmlResult {
    render(&Index {})
}

#[derive(Template)]
#[template(path = "about.html")]
struct About {}

async fn about() -> HtmlResult {
    render(&About {})
}

#[derive(Template)]
#[template(path = "paste.html")]
struct Paste {
    list: ListRow,
    html: String,
}

impl Paste {
    fn menu(&self) -> PasteMenu {
        PasteMenu::new(&self.list, PastePage::Highlight)
    }
}

async fn paste(
    Path(id): Path<String>,
    State(pool): State<Pool>,
    State(syntax): State<Syntax>,
) -> HtmlResult {
    let list = ListRow::find(&pool, &id).await?;
    let text = TextRow::find(&pool, &id).await?;

    let lang = &list.language.decode();
    let text = &text.text.decode();
    let html = syntax.highlight(lang, text)?;

    render(&Paste { list, html })
}

async fn raw(Path(id): Path<String>, State(pool): State<Pool>) -> TextResult {
    let row = TextRow::find(&pool, &id).await?;
    let text = row.text.decode().into_owned();

    Ok((StatusCode::OK, text))
}

async fn tree(Path(id): Path<String>, State(pool): State<Pool>) -> HtmlResult {
    let list = ListRow::find(&pool, &id).await?;
    let items = TreeItem::list(&pool, &id).await?;

    render(&TreeRoot::new(&list, &items))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/:id", get(paste))
        .route("/:id/raw", get(raw))
        .route("/:id/tree", get(tree))
}
