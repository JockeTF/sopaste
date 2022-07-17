use sea_orm_rocket::Database;

mod entities;
mod result;
mod r#static;
mod storage;
mod syntax;
mod templates;
// mod tree;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(storage::Pool::init())
        .manage(syntax::Syntect::new())
        .mount("/", r#static::routes())
        .mount("/", templates::routes())
}
