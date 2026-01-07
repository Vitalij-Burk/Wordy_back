use axum::{Json, extract::State, http::StatusCode};

use crate::AppState;
use crate::handlers::types::HandlerError;
use crate::models::user::{CreateUser, User};

#[axum::debug_handler]
pub async fn make_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, HandlerError> {
    let res = state
        .user_service
        .create(&payload)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Save failed"))?;

    Ok(Json(res))
}
