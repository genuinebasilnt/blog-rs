use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::routes::{health_check::health_check, subscribe::subscribe};

pub async fn run(db_pool: PgPool) -> Router {
    let connection = Arc::new(db_pool);
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection)
}
