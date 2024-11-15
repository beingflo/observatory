use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{auth::AuthenticatedUser, error::AppError, AppState};

#[tracing::instrument(skip_all)]
pub async fn get_gps_coords(
    State(state): State<AppState>,
    _: AuthenticatedUser,
    Path(bucket): Path<String>,
) -> Result<(StatusCode, String), AppError> {
    let conn = state.connection.lock().await;
    let mut stmt = conn
        .prepare("SELECT cast(payload -> '$.geometry.coordinates[0]' as float), cast(payload -> '$.geometry.coordinates[1]' as float) FROM timeseries WHERE bucket = (?) ORDER BY timestamp DESC;")?;

    let response: Result<Vec<GPSResponse>, _> = stmt
        .query_map([bucket], |row| {
            Ok(GPSResponse {
                longitude: row.get(0)?,
                latitude: row.get(1)?,
            })
        })?
        .collect();

    Ok((
        StatusCode::OK,
        response?
            .into_iter()
            .map(|r| format!("{}, {}", r.latitude, r.longitude))
            .collect::<Vec<String>>()
            .join("\n"),
    ))
}

#[derive(Debug, Serialize)]
pub struct GPSResponse {
    longitude: f64,
    latitude: f64,
}
