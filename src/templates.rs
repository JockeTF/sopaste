use std::result::Result;

use askama::Template;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::content::Custom;
use rocket::*;

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

pub fn routes() -> Vec<Route> {
    routes![index]
}
