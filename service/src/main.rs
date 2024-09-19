use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use duckdb::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

type StateType = Arc<Mutex<Connection>>;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Arc::new(Mutex::new(Connection::open("./db.duckdb")?));

    conn.lock().await.execute_batch(
        r"CREATE TABLE IF NOT EXISTS data (
            timestamp TIMESTAMP NOT NULL,
            bucket TEXT NOT NULL,
            payload JSON NOT NULL
          );
        ",
    )?;

    println!("Created table");

    let app = Router::new()
        .route("/", post(upload_data))
        .route("/", get(get_data))
        .route("/gps", post(upload_gps_data))
        .route("/gps", get(get_gps_coords))
        .with_state(conn);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Serialize)]
struct GPSResponse {
    longitude: f64,
    latitude: f64,
}

async fn get_gps_coords(State(conn): State<StateType>) -> (StatusCode, Json<Vec<GPSResponse>>) {
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
struct DataResponse {
    timestamp: String,
    payload: String,
}

async fn get_data(State(conn): State<StateType>) -> (StatusCode, Json<Vec<DataResponse>>) {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(timestamp as Text), payload FROM data ORDER BY timestamp DESC;")
        .unwrap();

    let response: Result<Vec<DataResponse>, _> = stmt
        .query_map([], |row| {
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: row.get(1)?,
            })
        })
        .unwrap()
        .collect();

    (StatusCode::OK, Json(response.unwrap()))
}

#[derive(Deserialize, Clone)]
struct Data {
    timestamp: Option<String>,
    bucket: String,
    payload: Value,
}

async fn upload_data(State(conn): State<StateType>, Json(request): Json<Data>) -> StatusCode {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("INSERT INTO data (timestamp, bucket, payload) VALUES (?, ?, ?);")
        .unwrap();
    let payload = serde_json::to_string(&request.payload).unwrap();
    stmt.execute(params![
        request.timestamp.unwrap_or(Utc::now().to_string()),
        request.bucket,
        payload
    ])
    .unwrap();

    StatusCode::OK
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
struct GPSData {
    locations: Vec<GPSLocation>,
}

async fn upload_gps_data(
    State(conn): State<StateType>,
    Json(payload): Json<GPSData>,
) -> StatusCode {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("INSERT INTO data (timestamp, bucket, payload) VALUES (?, ?, ?);")
        .unwrap();
    for location in payload.locations {
        let payload: String = serde_json::to_string(&location).unwrap();
        stmt.execute(params![location.properties.timestamp, "location", payload])
            .unwrap();
    }

    StatusCode::OK
}
