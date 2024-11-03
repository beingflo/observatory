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
pub async fn get_weight(
    _: AuthenticatedUser,
    Query(filters): Query<GetWeightFilters>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Weight>>), AppError> {
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
            "SELECT cast(timestamp as Text), cast(payload -> '$.weight' as Float) FROM timeseries WHERE bucket = 'weight-florian' AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) AS TIMESTAMP) ORDER BY timestamp DESC;",
        )?;

    let response: Result<Vec<Weight>, _> = stmt
        .query_map([from, to], |row| {
            Ok(Weight {
                timestamp: row.get(0)?,
                weight: row.get(1)?,
            })
        })?
        .collect();

    let mut response = response?;

    for d in response.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    Ok((StatusCode::OK, Json(response)))
}

#[derive(Deserialize)]
pub struct GetWeightFilters {
    from: Option<String>,
    to: Option<String>,
    past_days: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct WeightResponse {
    weight: Vec<Weight>,
    running_avg_weight: Vec<Weight>,
    number_weighins: u32,
    change_week_over_week: f32,
    change_month_over_month: f32,
}

#[derive(Debug, Serialize)]
pub struct Weight {
    timestamp: String,
    weight: f32,
}
