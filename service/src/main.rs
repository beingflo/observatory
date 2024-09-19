use std::{sync::Arc, time::Duration};

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
use tokio::{runtime::Handle, sync::Mutex, task};

type StateType = (Arc<Mutex<Connection>>, Arc<Mutex<Vec<Data>>>);

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Arc::new(Mutex::new(Connection::open("./db.duckdb")?));
    let buffer = Arc::new(Mutex::new(Vec::new()));

    conn.lock().await.execute_batch(
        r"CREATE TABLE IF NOT EXISTS data (
            timestamp TIMESTAMP NOT NULL,
            bucket TEXT NOT NULL,
            payload JSON NOT NULL
          );
        ",
    )?;

    println!("Created table");

    fn task(conn: Arc<Mutex<Connection>>, buffer: Arc<Mutex<Vec<Data>>>) {
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        loop {
            let handle = Handle::current();
            handle.block_on(interval.tick());

            let mut buffer = handle.block_on(buffer.lock());

            if buffer.len() < 1 {
                continue;
            }

            let buffer_len = buffer.len();
            let mut buffer_local: Vec<Data> =
                buffer.drain(0..std::cmp::min(1000, buffer_len)).collect();

            if buffer.len() == 0 {
                buffer.shrink_to_fit();
            }
            // Free up lock
            drop(buffer);

            let conn = handle.block_on(conn.lock());
            conn.execute_batch("BEGIN TRANSACTION").unwrap();
            let mut stmt = conn
                .prepare("INSERT INTO data (timestamp, bucket, payload) VALUES (?, ?, ?);")
                .unwrap();
            while let Some(p) = buffer_local.pop() {
                stmt.execute(params![
                    p.timestamp.unwrap_or(Utc::now().to_string()),
                    p.bucket,
                    p.payload.to_string(),
                ])
                .unwrap();
            }
            conn.execute_batch("COMMIT").unwrap();
        }
    }

    let conn_clone = conn.clone();
    let buffer_clone = buffer.clone();
    task::spawn_blocking(move || task(conn_clone, buffer_clone));

    let app = Router::new()
        .route("/", post(upload_data))
        .route("/", get(get_data))
        .route("/gps", post(upload_gps_data))
        .route("/gps", get(get_gps_coords))
        .with_state((conn, buffer));

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

async fn get_gps_coords(
    State((conn, _)): State<StateType>,
) -> (StatusCode, Json<Vec<GPSResponse>>) {
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

async fn get_data(State((conn, _)): State<StateType>) -> (StatusCode, Json<Vec<DataResponse>>) {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(timestamp as Text), payload FROM data;")
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

async fn upload_data(
    State((_, buffer)): State<StateType>,
    Json(payload): Json<Data>,
) -> StatusCode {
    let mut buffer = buffer.lock().await;

    buffer.push(payload);

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
    State((conn, _)): State<StateType>,
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
