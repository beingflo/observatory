use axum::{extract::State, http::StatusCode, Json};
use duckdb::params;
use serde::{Deserialize, Serialize};

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
        stmt.execute(params![location.properties.timestamp, "location", payload])
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
struct GPSProperties {
    timestamp: String,
    altitude: i32,
    speed: i32,
    horizontal_accuracy: i32,
    vertical_accuracy: i32,
    motion: [String; 2],
    pauses: bool,
    activity: String,
    desired_accuracy: i32,
    deferred: i32,
    significant_change: String,
    locations_in_payload: i32,
    device_id: String,
    wifi: String,
    battery_state: String,
    battery_level: f32,
}

#[derive(Deserialize, Serialize, Clone)]
struct GPSGeometry {
    r#type: String,
    coordinates: [f64; 2],
}

#[derive(Deserialize, Serialize, Clone)]
struct GPSLocation {
    properties: GPSProperties,
    r#type: String,
    geometry: GPSGeometry,
}

#[derive(Deserialize, Clone)]
pub struct GPSData {
    locations: Vec<GPSLocation>,
}
