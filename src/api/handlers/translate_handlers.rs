use axum::{
    Json,
    extract::{Path, State},
};
use reqwest::StatusCode;

use crate::{
    AppState,
    api::handlers::types::HandlerError,
    application::services::{
        translate_service::TranslateServiceError, word_pair_service::WordPairServiceError,
    },
    domain::models::{
        translate::ToTranslate,
        word_pair::{CreateWordPair, WordPair},
    },
};

#[axum::debug_handler]
async fn translate(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<ToTranslate>,
) -> Result<Json<WordPair>, HandlerError> {
    let target_text = state
        .translate_service
        .translate_text(
            &payload.source_text,
            &payload.target_language,
            &payload.source_language,
        )
        .await
        .map_err(|error| match error {
            TranslateServiceError::TranslatorError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Translation failed")
            }
            TranslateServiceError::NotFoundLanguage(_) => {
                (StatusCode::BAD_REQUEST, "Language not found")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let res = state
        .word_pair_service
        .create(
            &user_id,
            &CreateWordPair {
                target_text: target_text,
                source_text: payload.source_text,
                target_language: payload.target_language,
                source_language: payload.source_language,
            },
        )
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::WordPairAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Word pair already exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(Json(res))
}
