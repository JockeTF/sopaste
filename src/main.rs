#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::unused_async)]
#![allow(clippy::wildcard_imports)]

use axum::Router;
use axum::Server;
use config::Config;
use state::AppState;

mod config;
mod menu;
mod models;
mod result;
mod state;
mod r#static;
mod syntax;
mod templates;
mod tree;

fn routes(state: AppState) -> Router {
    Router::new()
        .merge(r#static::routes())
        .merge(templates::routes())
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let config = Config::new().unwrap();
    let state = AppState::from(config.database);
    let service = routes(state).into_make_service();

    Server::bind(&config.binding).serve(service).await.unwrap();
}
