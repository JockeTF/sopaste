use rocket::*;

mod r#static;

#[get("/")]
fn index() -> &'static str {
    "Hellopaca, World!"
}

#[launch]
fn initialize() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", r#static::routes())
}
