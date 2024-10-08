use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Days, Months, Utc};
use duckdb::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::StateType;

#[tracing::instrument(skip_all)]
pub async fn get_data(
    State(conn): State<StateType>,
    Query(filters): Query<GetDataFilters>,
) -> (StatusCode, Json<Vec<DataResponse>>) {
    let mut from = filters
        .from
        .unwrap_or(DateTime::from_timestamp_nanos(0).to_rfc3339());

    let mut to = filters.to.unwrap_or(
        DateTime::from_timestamp_nanos(0)
            .checked_add_months(Months::new(12_000))
            .unwrap()
            .to_rfc3339(),
    );

    if filters.past_days.is_some() {
        to = Utc::now().to_rfc3339();
        from = Utc::now()
            .checked_sub_days(Days::new(filters.past_days.unwrap() as u64))
            .unwrap()
            .to_rfc3339();
    }

    let limit = filters.limit.unwrap_or(100);

    let conn = conn.lock().await;
    let mut stmt;

    let response: Result<Vec<DataResponse>, _> = if filters.bucket.is_some() {
        stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload, bucket FROM data WHERE bucket = (?) AND timestamp > (?) AND timestamp < (?) ORDER BY timestamp DESC LIMIT (?);",
        )
        .unwrap();
        stmt.query_map(params![filters.bucket, from, to, limit], |row| {
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: row.get(1)?,
                bucket: row.get(2)?,
            })
        })
        .unwrap()
        .collect()
    } else {
        stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload, bucket FROM data WHERE timestamp > (?) AND timestamp < (?) ORDER BY timestamp DESC LIMIT (?);",
        )
        .unwrap();
        stmt.query_map(params![from, to, limit], |row| {
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: row.get(1)?,
                bucket: row.get(2)?,
            })
        })
        .unwrap()
        .collect()
    };

    (StatusCode::OK, Json(response.unwrap()))
}

#[tracing::instrument(skip_all)]
pub async fn delete_data(
    State(conn): State<StateType>,
    Query(filters): Query<DeleteDataFilters>,
) -> (StatusCode, Json<DataDeleteResponse>) {
    let mut from = filters
        .from
        .unwrap_or(DateTime::from_timestamp_nanos(0).to_rfc3339());

    let mut to = filters.to.unwrap_or(
        DateTime::from_timestamp_nanos(0)
            .checked_add_months(Months::new(12_000))
            .unwrap()
            .to_rfc3339(),
    );

    if filters.past_days.is_some() {
        to = Utc::now().to_rfc3339();
        from = Utc::now()
            .checked_sub_days(Days::new(filters.past_days.unwrap() as u64))
            .unwrap()
            .to_rfc3339();
    }

    let conn = conn.lock().await;
    let affected_rows;

    if filters.bucket.is_some() {
        affected_rows = conn
            .execute(
                "DELETE FROM data WHERE bucket = (?) AND timestamp > (?) AND timestamp < (?);",
                params![filters.bucket, from, to],
            )
            .unwrap();
    } else {
        affected_rows = conn
            .execute(
                "DELETE FROM data WHERE timestamp > (?) AND timestamp < (?);",
                params![from, to],
            )
            .unwrap();
    };

    info!(message = "Deleted rows", affected_rows);

    (StatusCode::OK, Json(DataDeleteResponse { affected_rows }))
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

#[derive(Deserialize)]
pub struct GetDataFilters {
    from: Option<String>,
    to: Option<String>,
    past_days: Option<u32>,
    limit: Option<u32>,
    bucket: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteDataFilters {
    from: Option<String>,
    to: Option<String>,
    past_days: Option<u32>,
    bucket: Option<String>,
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

#[derive(Debug, Serialize)]
pub struct DataDeleteResponse {
    affected_rows: usize,
}
