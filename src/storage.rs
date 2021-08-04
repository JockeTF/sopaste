use rocket::fairing::Fairing;

use rocket_db_pools::sqlx::MySqlPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("pastebin")]
pub struct Pool(MySqlPool);

pub fn fairing() -> impl Fairing {
    Pool::init()
}
