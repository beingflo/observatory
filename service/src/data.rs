use std::{collections::HashMap, str::FromStr, u32};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use duckdb::params;
use jiff::{Span, Timestamp, Zoned};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::{
    auth::{AuthenticatedEmitter, AuthenticatedUser},
    error::AppError,
    utils::sample,
    AppState,
};

#[tracing::instrument(skip_all)]
pub async fn get_data(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Query(filters): Query<DataFilter>,
) -> Result<(StatusCode, Json<Vec<DataResponse>>), AppError> {
    let mut from = if let Some(f) = filters.from {
        Timestamp::from_str(&f)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MIN.to_string()
    };

    let mut to = if let Some(t) = filters.to {
        Timestamp::from_str(&t)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MAX.to_string()
    };

    if let Some(past_days) = filters.past_days {
        to = Timestamp::now().to_string();
        from = Zoned::now()
            .checked_sub(Span::new().days(past_days))?
            .timestamp()
            .to_string();
    }

    let limit = filters.limit.unwrap_or(u32::MAX);

    let conn = state.connection.lock().await;
    let mut stmt;

    let response: Result<Vec<DataResponse>, _> = if let Some(bucket) = filters.bucket {
        stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload, bucket FROM timeseries WHERE bucket = (?) AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) AS TIMESTAMP) ORDER BY timestamp DESC LIMIT (?);",
        )?;
        stmt.query_map(params![bucket, from, to, limit], |row| {
            let payload: String = row.get(1)?;
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: serde_json::from_str(&payload).unwrap(),
                bucket: row.get(2)?,
            })
        })?
        .collect()
    } else {
        stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload, bucket FROM timeseries WHERE timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) as TIMESTAMP) ORDER BY timestamp DESC LIMIT (?);",
        )?;
        stmt.query_map(params![from, to, limit], |row| {
            let payload: String = row.get(1)?;
            Ok(DataResponse {
                timestamp: row.get(0)?,
                payload: serde_json::from_str(&payload).unwrap(),
                bucket: row.get(2)?,
            })
        })?
        .collect()
    };

    let mut response = response?;

    // Format dates in DB (can't be done in query_map due to error handling)
    for d in response.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    Ok((StatusCode::OK, Json(sample(filters.sample, response))))
}

#[tracing::instrument(skip_all)]
pub async fn delete_data(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Query(filters): Query<DeleteDataFilters>,
) -> Result<(StatusCode, Json<DataDeleteResponse>), AppError> {
    let mut from = if let Some(f) = filters.from {
        Timestamp::from_str(&f)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MIN.to_string()
    };

    let mut to = if let Some(t) = filters.to {
        Timestamp::from_str(&t)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string()
    } else {
        Timestamp::MAX.to_string()
    };

    if let Some(past_days) = filters.past_days {
        to = Timestamp::now().to_string();
        from = Zoned::now()
            .checked_sub(Span::new().days(past_days))?
            .timestamp()
            .to_string();
    }

    let conn = state.connection.lock().await;
    let affected_rows;

    if let Some(bucket) = filters.bucket {
        affected_rows = conn.execute(
            "DELETE FROM timeseries WHERE bucket = (?) AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) as TIMESTAMP);",
            params![bucket, from, to],
        )?;
    } else {
        affected_rows = conn.execute(
            "DELETE FROM timeseries WHERE timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) as TIMESTAMP);",
            params![from, to],
        )?
    };

    info!(message = "Deleted rows", affected_rows);

    Ok((StatusCode::OK, Json(DataDeleteResponse { affected_rows })))
}

#[tracing::instrument(skip_all, fields( emitter = %emitter.description))]
pub async fn upload_data(
    State(state): State<AppState>,
    emitter: AuthenticatedEmitter,
    Json(request): Json<Data>,
) -> Result<StatusCode, AppError> {
    let conn = state.connection.lock().await;
    let mut stmt =
        conn.prepare("INSERT INTO timeseries (timestamp, bucket, payload) VALUES (?, ?, ?);")?;
    let payload = serde_json::to_string(&request.payload)?;
    let timestamp = match request.timestamp {
        Some(ts) => Timestamp::from_str(&ts)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string(),
        None => Timestamp::now().to_string(),
    };
    stmt.execute(params![timestamp, request.bucket, payload])?;

    Ok(StatusCode::OK)
}

#[tracing::instrument(skip_all, fields( emitter = %emitter.description))]
pub async fn upload_data_url_only(
    State(state): State<AppState>,
    emitter: AuthenticatedEmitter,
    Path((_, bucket)): Path<(String, String)>,
    Query(data): Query<HashMap<String, String>>,
) -> Result<StatusCode, AppError> {
    let conn = state.connection.lock().await;
    let mut stmt =
        conn.prepare("INSERT INTO timeseries (timestamp, bucket, payload) VALUES (?, ?, ?);")?;

    let timestamp = match data.get("timestamp") {
        Some(ts) => Timestamp::from_str(&ts)
            .map_err(|e| AppError::DateInputError(e))?
            .to_string(),
        None => Timestamp::now().to_string(),
    };
    let payload = serde_json::to_string(&data)?;
    stmt.execute(params![timestamp, bucket, payload])?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct DataFilter {
    from: Option<String>,
    to: Option<String>,
    // past_days overrides `from` and `to` params
    past_days: Option<u32>,
    // return only last `limit` datapoints
    limit: Option<u32>,
    // sample at most ~`sample` datapoints from all otherwise returned
    sample: Option<u32>,
    // filter down datapoints to ones in `bucket`
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
    payload: Value,
}

#[derive(Debug, Serialize)]
pub struct DataDeleteResponse {
    affected_rows: usize,
}
