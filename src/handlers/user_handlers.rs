use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;
use crate::models::user::{CreateUser, User};
use crate::models::word_pair::WordPair;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::word_pair_repository::WordPairRepository;

#[axum::debug_handler]
pub async fn make_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<&'static str, (StatusCode, &'static str)> {
    let user = User::new(&payload.key, &payload.name);

    let repo = UserRepository { db: state.db };

    repo.save_user(user)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    Ok("User created")
}

#[axum::debug_handler]
pub async fn get_word_pairs(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WordPair>>, (StatusCode, &'static str)> {
    let repo = WordPairRepository { db: state.db };

    let res = repo
        .select_word_pairs(user_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database save failed"))?;

    for row in res.iter() {
        println!("Target text: {}", row.target_text);
        println!("Source text: {}", row.source_text);
    }

    println!("{:?}", res);

    Ok(Json(res))
}
