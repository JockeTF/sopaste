use rocket::*;

mod r#static;

#[get("/")]
fn index() -> &'static str {
    "Hellopaca, World!"
}

#[launch]
fn initialize() -> Rocket {
    ignite()
        .mount("/", routes![index])
        .mount("/static", r#static::routes())
}
