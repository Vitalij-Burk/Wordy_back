use axum::{Json, extract::State, http::StatusCode};

use crate::AppState;
use crate::api::handlers::types::HandlerError;
use crate::application::services::user_service::UserServiceError;
use crate::domain::models::user::{CreateUser, User};

#[axum::debug_handler]
pub async fn make_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, HandlerError> {
    let res = state
        .user_service
        .create(&payload)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::InvalidKey(_) => (StatusCode::UNPROCESSABLE_ENTITY, "Key is invalid"),
            UserServiceError::InvalidPassword(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Password is too easy")
            }
            UserServiceError::KeyAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Key already exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(Json(res))
}
