use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod handlers;
mod models;
mod repositories;
mod services;
mod translate;

use handlers::user_handlers::make_user;
use handlers::word_pair_handlers::{
    add_word_pair_by_user_id, add_word_pair_by_user_key, get_word_pairs_by_user_id,
    get_word_pairs_by_user_key,
};

use crate::{
    repositories::{user_repository::UserRepository, word_pair_repository::WordPairRepository},
    services::{user_service::UserService, word_pair_service::WordPairService},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub user_service: UserService<UserRepository>,
    pub word_pair_service: WordPairService<WordPairRepository>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let user_repo = UserRepository { db: db.clone() };
        let word_pair_repo = WordPairRepository { db: db.clone() };

        let user_service = UserService::new(user_repo);
        let word_pair_service = WordPairService::new(word_pair_repo);

        Self {
            db: db,
            user_service: user_service,
            word_pair_service: word_pair_service,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let pool = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    let state = AppState::new(pool);
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/user/create/", post(make_user))
        .route(
            "/user/user_id/{user_id}/wordpair/create/",
            post(add_word_pair_by_user_id),
        )
        .route(
            "/user/key/{key}/wordpair/create/",
            post(add_word_pair_by_user_key),
        )
        .route(
            "/user/user_id/{user_id}/wordpairs/",
            get(get_word_pairs_by_user_id),
        )
        .route(
            "/user/key/{key}/wordpairs/",
            get(get_word_pairs_by_user_key),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
