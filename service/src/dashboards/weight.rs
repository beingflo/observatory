use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};
use jiff::Timestamp;
use serde::Serialize;

use crate::{error::AppError, StateType};

#[tracing::instrument(skip_all)]
pub async fn get_weight(
    State(conn): State<StateType>,
) -> Result<(StatusCode, Json<Vec<Weight>>), AppError> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload -> '$.weight' FROM timeseries WHERE bucket = 'weight' ORDER BY timestamp DESC;",
        )?;

    let response: Result<Vec<Weight>, _> = stmt
        .query_map([], |row| {
            Ok(Weight {
                timestamp: row.get(0)?,
                weight: row.get(1)?,
            })
        })?
        .collect();

    let mut response = response?;

    for d in response.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Serialize)]
pub struct Weight {
    timestamp: String,
    weight: f32,
}
