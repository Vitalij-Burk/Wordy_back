use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;
use crate::handlers::types::HandlerError;
use crate::models::word_pair::{CreateWordPair, WordPair};

#[axum::debug_handler]
pub async fn add_word_pair_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateWordPair>,
) -> Result<Json<WordPair>, HandlerError> {
    let word_pair = state
        .word_pair_service
        .create(&user_id, &payload)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(word_pair))
}

#[axum::debug_handler]
pub async fn add_word_pair_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(payload): Json<CreateWordPair>,
) -> Result<Json<WordPair>, HandlerError> {
    let user = state
        .user_service
        .get_by_key(&key)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database get failed"))?;

    let word_pair = state
        .word_pair_service
        .create(&user.id, &payload)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(word_pair))
}

#[axum::debug_handler]
pub async fn get_word_pairs_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WordPair>>, HandlerError> {
    let res = state
        .word_pair_service
        .get_by_user_id(&user_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_word_pairs_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<Vec<WordPair>>, HandlerError> {
    let user = state
        .user_service
        .get_by_key(&key)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database get failed"))?;

    let res = state
        .word_pair_service
        .get_by_user_id(&user.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok(Json(res))
}
