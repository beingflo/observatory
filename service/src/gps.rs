use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use duckdb::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::StateType;

#[tracing::instrument(skip_all)]
pub async fn upload_gps_data(
    State(conn): State<StateType>,
    Json(payload): Json<GPSData>,
) -> Json<GPSUploadResponse> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("INSERT INTO data (timestamp, bucket, payload) VALUES (?, ?, ?);")
        .unwrap();
    for location in payload.locations {
        let payload: String = serde_json::to_string(&location).unwrap();
        let timestamp: String = match location.properties["timestamp"].as_str() {
            Some(ts) => DateTime::parse_from_rfc3339(ts)
                .unwrap()
                .naive_utc()
                .to_string(),
            None => Utc::now().to_string(),
        };

        stmt.execute(params![timestamp, "location", payload])
            .unwrap();
    }

    Json(GPSUploadResponse {
        result: "ok".into(),
    })
}

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(State(conn): State<StateType>) -> (StatusCode, Json<Vec<GPSResponse>>) {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float) FROM data WHERE bucket = 'location';")
        .unwrap();

    let response: Result<Vec<GPSResponse>, _> = stmt
        .query_map([], |row| {
            Ok(GPSResponse {
                longitude: row.get(0)?,
                latitude: row.get(1)?,
            })
        })
        .unwrap()
        .collect();

    (StatusCode::OK, Json(response.unwrap()))
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
