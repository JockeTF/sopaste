#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::wildcard_imports)]

use rocket_db_pools::Database;

mod models;
mod result;
mod r#static;
mod storage;
mod syntax;
mod templates;
mod tree;

#[rocket::launch]
fn initialize() -> _ {
    rocket::build()
        .attach(storage::Pool::init())
        .manage(syntax::Syntect::new())
        .mount("/", r#static::routes())
        .mount("/", templates::routes())
}
