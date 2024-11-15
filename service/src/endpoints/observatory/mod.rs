use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};
use jiff::Timestamp;
use serde::Serialize;

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_observatory_info(
    _: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ObservatoryInfoResponse>), AppError> {
    let conn = state.connection.lock().await;
    let mut stmt = conn.prepare(
        "SELECT cast(timestamp as Text), bucket FROM timeseries ORDER BY timestamp DESC;",
    )?;

    let response: Result<Vec<DataPoint>, _> = stmt
        .query_map([], |row| {
            Ok(DataPoint {
                timestamp: row.get(0)?,
                bucket: row.get(1)?,
            })
        })?
        .collect();

    let mut response = response?;

    for d in response.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    Ok((
        StatusCode::OK,
        Json(ObservatoryInfoResponse {
            data_points: response,
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct ObservatoryInfoResponse {
    data_points: Vec<DataPoint>,
}

#[derive(Debug, Serialize)]
pub struct DataPoint {
    timestamp: String,
    bucket: String,
}
