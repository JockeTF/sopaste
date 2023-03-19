use axum::extract::FromRef;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::MySqlPool;
use std::sync::Arc;

use crate::syntax::Syntect;

pub type Pool = MySqlPool;
pub type Syntax = Arc<Syntect>;

#[derive(Clone)]
pub struct AppState {
    storage: Pool,
    syntax: Syntax,
}

impl From<MySqlConnectOptions> for AppState {
    fn from(value: MySqlConnectOptions) -> Self {
        AppState {
            storage: Pool::connect_lazy_with(value),
            syntax: Arc::new(Syntect::default()),
        }
    }
}

impl FromRef<AppState> for Pool {
    fn from_ref(input: &AppState) -> Self {
        input.storage.clone()
    }
}

impl FromRef<AppState> for Syntax {
    fn from_ref(input: &AppState) -> Self {
        input.syntax.clone()
    }
}
