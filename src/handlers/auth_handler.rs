use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, models::{auth::{RegisterRequest, RegisterResponse}},};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {

    request.validate().map_err(|e| StatusCode::BAD_REQUEST)?;
    
    //let user = auth_service::register_user(user_repo, email, password).await?;
    
    Ok(Json(RegisterResponse {
        id: Uuid::new_v4().to_string(),
        email: request.email,
        created_at: Utc::now().to_string(),
    }))
}