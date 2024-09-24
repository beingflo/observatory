use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use duckdb::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::StateType;

#[tracing::instrument(skip_all)]
pub async fn get_data(State(conn): State<StateType>) -> (StatusCode, Json<Vec<DataResponse>>) {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload, bucket FROM data ORDER BY timestamp DESC;",
        )
        .unwrap();

    let response: Result<Vec<DataResponse>, _> = stmt
        .query_map([], |row| {
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: row.get(1)?,
                bucket: row.get(2)?,
            })
        })
        .unwrap()
        .collect();

    (StatusCode::OK, Json(response.unwrap()))
}

#[tracing::instrument(skip_all)]
pub async fn upload_data(State(conn): State<StateType>, Json(request): Json<Data>) -> StatusCode {
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

#[derive(Deserialize, Clone)]
pub struct Data {
    timestamp: Option<String>,
    bucket: String,
    payload: Value,
}

#[derive(Debug, Serialize)]
pub struct DataResponse {
    timestamp: String,
    bucket: String,
    payload: String,
}
