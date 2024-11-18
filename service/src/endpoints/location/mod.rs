use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use duckdb::params;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Path(bucket): Path<String>,
    Query(filters): Query<DataFilter>,
) -> Result<(StatusCode, Json<Vec<GPSResponse>>), AppError> {
    let from = if let Some(f) = filters.from {
        Timestamp::from_str(&f)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MIN.to_string()
    };

    let to = if let Some(t) = filters.to {
        Timestamp::from_str(&t)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MAX.to_string()
    };
    let limit = filters.limit.unwrap_or(u32::MAX);

    let conn = state.connection.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float), cast(timestamp as Text) FROM timeseries WHERE bucket = (?) AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) as TIMESTAMP) ORDER BY timestamp DESC LIMIT (?);")?;

    let response: Result<Vec<GPSResponse>, _> = stmt
        .query_map(params![bucket, from, to, limit], |row| {
            Ok(GPSResponse {
                longitude: row.get(0)?,
                latitude: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?
        .collect();

    Ok((StatusCode::OK, Json(response?)))
}

#[derive(Debug, Serialize)]
pub struct GPSResponse {
    longitude: f64,
    latitude: f64,
    timestamp: String,
}

#[derive(Deserialize)]
pub struct DataFilter {
    from: Option<String>,
    to: Option<String>,
    // return only last `limit` datapoints
    limit: Option<u32>,
}
