use std::str::FromStr;

use axum::{
    extract::{Path, State},
    Json,
};
use duckdb::params;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{auth::AuthenticatedEmitter, error::AppError, AppState};

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

#[derive(Debug, Serialize)]
pub struct GPSUploadResponse {
    result: String,
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
