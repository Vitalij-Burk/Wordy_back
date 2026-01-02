use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod handlers;
mod models;
mod repositories;
mod translate;

use handlers::user_handlers::{get_word_pairs, make_user};
use handlers::word_pair_handlers::add_word_pair;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:123321@172.17.0.2/wordy-db")
        .await?;
    let state = AppState { db: pool };
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/user/create/", post(make_user))
        .route("/user/{user_id}/wordpair/create/", post(add_word_pair))
        .route("/user/{user_id}/wordpairs/", get(get_word_pairs))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
