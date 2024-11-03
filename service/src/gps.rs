use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use duckdb::params;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    auth::{AuthenticatedEmitter, AuthenticatedUser},
    error::AppError,
    AppState,
};

#[tracing::instrument(skip_all, fields( emitter = %emitter.description))]
pub async fn upload_gps_data(
    State(state): State<AppState>,
    emitter: AuthenticatedEmitter,
    Path((_, bucket)): Path<(String, String)>,
    Json(payload): Json<GPSData>,
) -> Result<Json<GPSUploadResponse>, AppError> {
    let conn = state.connection.lock().await;
    let mut stmt =
        conn.prepare("INSERT INTO timeseries (timestamp, bucket, payload) VALUES (?, ?, ?);")?;
    for location in payload.locations {
        let payload: String = serde_json::to_string(&location)?;
        let timestamp = match location.properties["timestamp"].as_str() {
            Some(ts) => Timestamp::from_str(ts)
                .map_err(|e| AppError::DateInputError(e))?
                .to_string(),
            None => Timestamp::now().to_string(),
        };

        stmt.execute(params![timestamp, bucket, payload])?;
    }

    Ok(Json(GPSUploadResponse {
        result: "ok".into(),
    }))
}

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Path(bucket): Path<String>,
) -> Result<(StatusCode, String), AppError> {
    let conn = state.connection.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float) FROM timeseries WHERE bucket = (?) ORDER BY timestamp DESC;")?;

    let response: Result<Vec<GPSResponse>, _> = stmt
        .query_map([bucket], |row| {
            Ok(GPSResponse {
                longitude: row.get(0)?,
                latitude: row.get(1)?,
            })
        })?
        .collect();

    Ok((
        StatusCode::OK,
        response?
            .into_iter()
            .map(|r| format!("{}, {}", r.latitude, r.longitude))
            .collect::<Vec<String>>()
            .join("\n"),
    ))
}

#[derive(Debug, Serialize)]
pub struct GPSUploadResponse {
    result: String,
}

#[derive(Debug, Serialize)]
pub struct GPSResponse {
    longitude: f64,
    latitude: f64,
}

#[derive(Deserialize, Serialize, Clone)]
struct GPSGeometry {
    r#type: String,
    coordinates: [f64; 2],
}

#[derive(Deserialize, Serialize, Clone)]
struct GPSLocation {
    properties: Value,
    r#type: String,
    geometry: GPSGeometry,
}

#[derive(Deserialize, Clone)]
pub struct GPSData {
    locations: Vec<GPSLocation>,
}
