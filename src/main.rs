#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use axum::Router;
use config::Config;
use state::AppState;
use tokio::net::TcpListener;

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
        .fallback(result::fallback)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let config = Config::new().unwrap();
    let router = routes(AppState::from(config.database));
    let listener = TcpListener::bind(config.binding).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
