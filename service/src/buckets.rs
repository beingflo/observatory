use axum::{extract::State, Json};

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_distinct_buckets(
    State(state): State<AppState>,
    _: AuthenticatedUser,
) -> Result<Json<Vec<String>>, AppError> {
    let conn = state.connection.lock().await;

    let mut stmt = conn.prepare("SELECT DISTINCT bucket FROM timeseries;")?;
    let response: Result<Vec<String>, _> = stmt.query_map([], |row| Ok(row.get(0)?))?.collect();

    let response = response?;

    Ok(Json(response))
}
