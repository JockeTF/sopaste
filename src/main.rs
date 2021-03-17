use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Hellopaca, World!"
}

#[launch]
fn initialize() -> Rocket {
    ignite().mount("/", routes![index])
}
