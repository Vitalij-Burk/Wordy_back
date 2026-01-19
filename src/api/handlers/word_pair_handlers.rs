use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::api::handlers::types::HandlerError;
use crate::domain::models::word_pair::{CreateWordPair, WordPair};
use crate::{
    AppState,
    application::services::{
        user_service::UserServiceError, word_pair_service::WordPairServiceError,
    },
};

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
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::WordPairAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Word pair exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

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
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let word_pair = state
        .word_pair_service
        .create(&user.id, &payload)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::WordPairAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Word pair exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

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
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

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
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let res = state
        .word_pair_service
        .get_by_user_id(&user.id)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(Json(res))
}
