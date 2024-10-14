use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::{error::AppError, StateType};

#[tracing::instrument(skip_all)]
pub async fn get_weight(
    State(conn): State<StateType>,
) -> Result<(StatusCode, Json<Vec<Weight>>), AppError> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload -> '$.weight' FROM data WHERE bucket = 'weight' ORDER BY timestamp DESC;",
        )?;

    let response: Result<Vec<Weight>, _> = stmt
        .query_map([], |row| {
            Ok(Weight {
                timestamp: row.get(0)?,
                weight: row.get(1)?,
            })
        })?
        .collect();

    Ok((StatusCode::OK, Json(response.unwrap())))
}

#[derive(Debug, Serialize)]
pub struct Weight {
    timestamp: String,
    weight: f32,
}
