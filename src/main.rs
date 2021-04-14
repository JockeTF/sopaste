mod r#static;
mod templates;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .mount("/", templates::routes())
        .mount("/static", r#static::routes())
}
