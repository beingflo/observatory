use std::collections::HashMap;

use crate::{error::AppError, AppState};
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::{error, info};

pub struct AuthenticatedEmitter {
    pub description: String,
}

pub struct AuthenticatedUser {}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedEmitter {
    type Rejection = AppError;

    #[tracing::instrument(skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let connection = state.connection.lock().await;

        let path: Path<HashMap<String, String>> = match Path::from_request_parts(parts, state).await
        {
            Ok(path) => path,
            Err(_) => return Err(AppError::Status(StatusCode::INTERNAL_SERVER_ERROR)),
        };

        let headers = match HeaderMap::from_request_parts(parts, state).await {
            Ok(headers) => headers,
            Err(_) => return Err(AppError::Status(StatusCode::INTERNAL_SERVER_ERROR)),
        };

        let token = match headers.get("emitter") {
            Some(token) => token
                .to_str()
                .map_err(|_| AppError::Status(StatusCode::BAD_REQUEST))
                .unwrap(),
            None => {
                info!(message = "Missing token header");
                match path.get("emitter") {
                    Some(value) => value,
                    None => {
                        error!(message = "Missing emitter path");
                        return Err(AppError::Status(StatusCode::UNAUTHORIZED));
                    }
                }
            }
        };

        let mut stmt = connection.prepare(
            "
                SELECT description 
                FROM emitters 
                WHERE token = ?
            ",
        )?;

        let mut rows = stmt.query([token])?;

        let emitter = match rows.next()? {
            Some(row) => AuthenticatedEmitter {
                description: row.get(0)?,
            },
            None => {
                error!(message = "No emittor found for token");
                return Err(AppError::Status(StatusCode::UNAUTHORIZED));
            }
        };

        return Ok(emitter);
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    #[tracing::instrument(skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let headers = match HeaderMap::from_request_parts(parts, state).await {
            Ok(headers) => headers,
            Err(_) => {
                return Err(AppError::Status(StatusCode::INTERNAL_SERVER_ERROR).into_response())
            }
        };

        let mut response_headers = HeaderMap::new();
        response_headers.insert(
            "WWW-Authenticate",
            HeaderValue::from_str(r#"Basic realm="observatory""#).unwrap(),
        );

        let auth = match headers.get("Authorization") {
            Some(token) => token
                .to_str()
                .map_err(|_| AppError::Status(StatusCode::BAD_REQUEST))
                .unwrap(),
            None => {
                error!(message = "Missing auth header");
                return Err((StatusCode::UNAUTHORIZED, response_headers).into_response());
            }
        };

        if auth.split(" ").nth(1).unwrap_or("invalid") == state.admin_auth {
            return Ok(AuthenticatedUser {});
        }

        return Err((StatusCode::UNAUTHORIZED, response_headers).into_response());
    }
}
