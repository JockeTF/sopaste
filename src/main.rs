mod models;
mod result;
mod r#static;
mod storage;
mod templates;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(storage::fairing())
        .mount("/", templates::routes())
        .mount("/static", r#static::routes())
}
