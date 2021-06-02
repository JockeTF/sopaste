mod models;
mod result;
mod r#static;
mod storage;
mod syntax;
mod templates;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(storage::fairing())
        .manage(syntax::Syntect::new())
        .mount("/", templates::routes())
        .mount("/static", r#static::routes())
}
