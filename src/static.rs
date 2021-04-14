use rocket::http::ContentType;
use rocket::response::Content;
use rocket::*;

#[get("/logo.png")]
fn logo() -> Content<&'static [u8]> {
    Content(ContentType::PNG, include_bytes!("../static/logo.png"))
}

#[get("/shadow.png")]
fn shadow() -> Content<&'static [u8]> {
    Content(ContentType::PNG, include_bytes!("../static/shadow.png"))
}

#[get("/style.css")]
fn style() -> Content<&'static str> {
    Content(ContentType::CSS, include_str!("../static/style.css"))
}

pub fn routes() -> Vec<Route> {
    routes![logo, shadow, style]
}
