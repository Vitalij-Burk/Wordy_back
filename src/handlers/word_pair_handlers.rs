use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{AppState, services::word_pair_service::WordPairService};
use crate::{
    models::word_pair::{CreateWordPair, WordPair},
    repositories::word_pair_repository::WordPairRepository,
};

#[axum::debug_handler]
pub async fn add_word_pair(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateWordPair>,
) -> Result<Json<WordPair>, (StatusCode, &'static str)> {
    let repo = WordPairRepository { db: state.db };

    let word_pair_service = WordPairService::new(repo);

    let word_pair = word_pair_service
        .create(&user_id, &payload)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(word_pair))
}

#[axum::debug_handler]
pub async fn get_word_pairs(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WordPair>>, (StatusCode, &'static str)> {
    let repo = WordPairRepository { db: state.db };

    let word_pair_service = WordPairService::new(repo);

    let res = word_pair_service
        .get_by_user_id(&user_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(res))
}
