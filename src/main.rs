use rocket_db_pools::Database;

mod models;
mod result;
mod r#static;
mod storage;
mod syntax;
mod templates;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(storage::Pool::init())
        .manage(syntax::Syntect::new())
        .mount("/", r#static::routes())
        .mount("/", templates::routes())
}
