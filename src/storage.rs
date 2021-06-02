use rocket::error;
use rocket::fairing::AdHoc;
use rocket::fairing::Fairing;
use rocket::fairing::Result;
use rocket::Build;
use rocket::Rocket;
use rocket::State;

use rocket_sync_db_pools::Config;

use sqlx::mysql::MySqlPool;

pub type Pool = State<MySqlPool>;

async fn connect(rocket: Rocket<Build>) -> Result {
    let url = match Config::from("pastebin", &rocket) {
        Ok(config) => config.url,
        Err(error) => {
            error!("{}", error);
            return Err(rocket);
        }
    };

    let pool = match MySqlPool::connect(&url).await {
        Ok(pool) => pool,
        Err(error) => {
            error!("{}", error);
            return Err(rocket);
        }
    };

    Ok(rocket.manage(pool))
}

pub fn fairing() -> impl Fairing {
    AdHoc::try_on_ignite("Pool", connect)
}
