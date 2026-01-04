use axum::{Json, extract::State, http::StatusCode};

use crate::models::user::{CreateUser, User};
use crate::repositories::user_repository::UserRepository;
use crate::{AppState, services::user_service::UserService};

#[axum::debug_handler]
pub async fn make_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
    let repo = UserRepository { db: state.db };

    let user_service = UserService::new(repo);

    let res = user_service
        .create(&payload)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Save failed"))?;

    Ok(Json(res))
}
