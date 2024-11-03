use std::str::FromStr;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use jiff::{Span, Timestamp, Unit, Zoned};
use serde::{Deserialize, Serialize};

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_weight(
    _: AuthenticatedUser,
    Query(filters): Query<GetWeightFilters>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<WeightResponse>), AppError> {
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
            "SELECT cast(timestamp as Text), cast(payload -> '$.weight' as Float) FROM timeseries WHERE bucket = 'weight-florian' AND timestamp > CAST((?) as TIMESTAMP) AND timestamp < CAST((?) AS TIMESTAMP) ORDER BY timestamp ASC;",
        )?;

    let weights: Result<Vec<Weight>, _> = stmt
        .query_map([from, to], |row| {
            Ok(Weight {
                timestamp: row.get(0)?,
                weight: row.get(1)?,
            })
        })?
        .collect();

    let mut weights = weights?;

    for d in weights.iter_mut() {
        d.timestamp = Timestamp::from_str(&d.timestamp)?.to_string();
    }

    let number_weighins = weights.len() as u32;

    let mut smooth_weights = Vec::new();
    let mut last = weights[0].weight;
    let alpha = 0.3;
    for d in weights.iter() {
        last = alpha * d.weight + (1.0 - alpha) * last;
        smooth_weights.push(Weight {
            timestamp: d.timestamp.clone(),
            weight: last,
        });
    }

    let mut change_percent_month_over_month = None;
    let mut change_percent_week_over_week = None;

    if weights.first().is_some() && weights.last().is_some() {
        let first = Timestamp::from_str(&weights.first().unwrap().timestamp)?;
        let last = Timestamp::from_str(&weights.last().unwrap().timestamp)?;

        if (last - first).total(Unit::Day)? >= 14.0 {
            let weights_last_week: Vec<f32> = weights
                .iter()
                .filter(|weight| {
                    if let Ok(ts) = Timestamp::from_str(&weight.timestamp) {
                        if (last - ts).total(Unit::Day).unwrap_or(0.0) > 7.0
                            && (last - ts).total(Unit::Day).unwrap_or(0.0) < 14.0
                        {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|weight| weight.weight)
                .collect();
            let weights_this_week: Vec<f32> = weights
                .iter()
                .filter(|weight| {
                    if let Ok(ts) = Timestamp::from_str(&weight.timestamp) {
                        if (last - ts).total(Unit::Day).unwrap_or(0.0) < 7.0 {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|weight| weight.weight)
                .collect();

            if weights_this_week.len() > 0 && weights_last_week.len() > 0 {
                let avg_this_week =
                    weights_this_week.iter().sum::<f32>() / weights_this_week.len() as f32;
                let avg_last_week =
                    weights_last_week.iter().sum::<f32>() / weights_last_week.len() as f32;
                change_percent_week_over_week = Some(100.0 * (1.0 - avg_this_week / avg_last_week));
            }
        }
        if (last - first).total(Unit::Day)? >= 60.0 {
            let weights_last_month: Vec<f32> = weights
                .iter()
                .filter(|weight| {
                    if let Ok(ts) = Timestamp::from_str(&weight.timestamp) {
                        if (last - ts).total(Unit::Day).unwrap_or(0.0) > 30.0
                            && (last - ts).total(Unit::Day).unwrap_or(0.0) < 60.0
                        {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|weight| weight.weight)
                .collect();
            let weights_this_month: Vec<f32> = weights
                .iter()
                .filter(|weight| {
                    if let Ok(ts) = Timestamp::from_str(&weight.timestamp) {
                        if (last - ts).total(Unit::Day).unwrap_or(0.0) < 30.0 {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|weight| weight.weight)
                .collect();

            if weights_this_month.len() > 0 && weights_last_month.len() > 0 {
                let avg_last_month =
                    weights_this_month.iter().sum::<f32>() / weights_this_month.len() as f32;
                let avg_this_month =
                    weights_last_month.iter().sum::<f32>() / weights_last_month.len() as f32;
                change_percent_month_over_month =
                    Some(100.0 * (1.0 - avg_this_month / avg_last_month));
            }
        }
    }

    Ok((
        StatusCode::OK,
        Json(WeightResponse {
            weights: weights,
            smooth_weights,
            number_weighins,
            change_percent_month_over_month,
            change_percent_week_over_week,
        }),
    ))
}

#[derive(Deserialize)]
pub struct GetWeightFilters {
    from: Option<String>,
    to: Option<String>,
    past_days: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct WeightResponse {
    weights: Vec<Weight>,
    smooth_weights: Vec<Weight>,
    number_weighins: u32,
    change_percent_week_over_week: Option<f32>,
    change_percent_month_over_month: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct Weight {
    timestamp: String,
    weight: f32,
}
