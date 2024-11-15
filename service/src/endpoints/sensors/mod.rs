use std::str::FromStr;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use jiff::{Span, Timestamp, Zoned};
use serde::{Deserialize, Serialize};

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_co2(
    _: AuthenticatedUser,
    Query(filters): Query<GetHomeDataFilters>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<CO2Response>), AppError> {
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
    let mut stmt = conn
        .prepare(
            "SELECT cast(timestamp as Text), cast(payload -> '$.co2' as Integer) FROM timeseries WHERE bucket = 'co2-sensor-living-room' AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) AS TIMESTAMP) ORDER BY timestamp ASC;",
        )?;

    let data: Result<Vec<DataPoint>, _> = stmt
        .query_map([from, to], |row| {
            Ok(DataPoint {
                timestamp: row.get(0)?,
                co2: row.get(1)?,
            })
        })?
        .collect();

    let mut data = data?;

    for d in data.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    Ok((StatusCode::OK, Json(CO2Response { data })))
}

#[derive(Deserialize)]
pub struct GetHomeDataFilters {
    from: Option<String>,
    to: Option<String>,
    past_days: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CO2Response {
    data: Vec<DataPoint>,
}

#[derive(Debug, Serialize)]
pub struct DataPoint {
    timestamp: String,
    co2: u32,
}
