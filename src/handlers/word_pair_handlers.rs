use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;
use crate::models::word_pair::CreateWordPair;
use crate::repositories::word_pair_repository::WordPairRepository;
use crate::translate::translate::translate_text;

#[axum::debug_handler]
pub async fn add_word_pair(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateWordPair>,
) -> Result<&'static str, (StatusCode, &'static str)> {
    let word_pair = translate_text(
        &user_id,
        &payload.source_text,
        &payload.source_language,
        &payload.target_language,
    )
    .await
    .unwrap();

    let repo = WordPairRepository { db: state.db };

    repo.save_word_pair(word_pair)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok("Added word pair")
}
