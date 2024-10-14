use axum::{extract::State, http::StatusCode, Json};
use duckdb::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::AppError, StateType};

#[tracing::instrument(skip_all)]
pub async fn upload_gps_data(
    State(conn): State<StateType>,
    Json(payload): Json<GPSData>,
) -> Result<Json<GPSUploadResponse>, AppError> {
    let conn = conn.lock().await;
    let mut stmt =
        conn.prepare("INSERT INTO data (timestamp, bucket, payload) VALUES (?, ?, ?);")?;
    for location in payload.locations {
        let payload: String = serde_json::to_string(&location).unwrap();
        let timestamp = location.properties["timestamp"].as_str().unwrap();

        stmt.execute(params![timestamp, "location", payload])?;
    }

    Ok(Json(GPSUploadResponse {
        result: "ok".into(),
    }))
}

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(
    State(conn): State<StateType>,
) -> Result<(StatusCode, String), AppError> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float) FROM data WHERE bucket = 'location' ORDER BY timestamp DESC;")?;

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
        response
            .unwrap()
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
