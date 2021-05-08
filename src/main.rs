mod models;
mod r#static;
mod templates;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(models::fairing())
        .mount("/", templates::routes())
        .mount("/static", r#static::routes())
}
