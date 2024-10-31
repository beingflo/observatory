use axum::{extract::State, http::StatusCode, Json};
use duckdb::params;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{auth::AuthenticatedUser, error::AppError, utils::get_auth_token, AppState};

#[tracing::instrument(skip_all, fields( emitter = %request.description))]
pub async fn add_emitter(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Json(request): Json<AddEmitterRequest>,
) -> Result<Json<AddEmitterResponse>, AppError> {
    let conn = state.connection.lock().await;

    let mut stmt = conn.prepare("SELECT count(*) FROM emitters WHERE description = (?);")?;
    let mut rows = stmt.query(params![request.description])?;
    let count = if let Some(row) = rows.next()? {
        row.get(0)?
    } else {
        0
    };

    if count > 0 {
        return Err(AppError::Status(StatusCode::BAD_REQUEST));
    }

    let mut stmt = conn.prepare("INSERT INTO emitters (token, description) VALUES (?, ?);")?;
    let token = get_auth_token();
    stmt.execute(params![token, request.description])?;

    Ok(Json(AddEmitterResponse {
        token,
        description: request.description,
    }))
}

#[tracing::instrument(skip_all, fields( emitter = %request.description))]
pub async fn delete_emitter(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Json(request): Json<DeleteEmitterRequest>,
) -> Result<StatusCode, AppError> {
    let conn = state.connection.lock().await;

    let affected_rows = conn.execute(
        "DELETE FROM emitters WHERE description = (?);",
        params![request.description],
    )?;

    info!(message = "Deleted rows", affected_rows);

    if affected_rows == 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::OK)
    }
}

#[derive(Deserialize)]
pub struct DeleteEmitterRequest {
    description: String,
}

#[derive(Deserialize)]
pub struct AddEmitterRequest {
    description: String,
}

#[derive(Debug, Serialize)]
pub struct AddEmitterResponse {
    description: String,
    token: String,
}
