use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};
use duckdb::params;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    auth::{AuthenticatedEmitter, AuthenticatedUser},
    error::AppError,
    StateType,
};

#[tracing::instrument(skip_all, fields( emitter = %emitter.description))]
pub async fn upload_gps_data(
    State(conn): State<StateType>,
    emitter: AuthenticatedEmitter,
    Json(payload): Json<GPSData>,
) -> Result<Json<GPSUploadResponse>, AppError> {
    let conn = conn.lock().await;
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

        stmt.execute(params![timestamp, "location", payload])?;
    }

    Ok(Json(GPSUploadResponse {
        result: "ok".into(),
    }))
}

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(
    State(conn): State<StateType>,
    _: AuthenticatedUser,
) -> Result<(StatusCode, String), AppError> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float) FROM timeseries WHERE bucket = 'location' ORDER BY timestamp DESC;")?;

    let response: Result<Vec<GPSResponse>, _> = stmt
        .query_map([], |row| {
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
