use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::StateType;

#[tracing::instrument(skip_all)]
pub async fn get_weight(State(conn): State<StateType>) -> (StatusCode, Json<Vec<Weight>>) {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), payload -> '$.weight' FROM data WHERE bucket = 'weight' ORDER BY timestamp DESC;",
        )
        .unwrap();

    let response: Result<Vec<Weight>, _> = stmt
        .query_map([], |row| {
            Ok(Weight {
                timestamp: row.get(0)?,
                weight: row.get(1)?,
            })
        })
        .unwrap()
        .collect();

    (StatusCode::OK, Json(response.unwrap()))
}

#[derive(Debug, Serialize)]
pub struct Weight {
    timestamp: String,
    weight: i32,
}
