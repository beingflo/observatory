use crate::{error::AppError, StateType};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::error;

pub struct AuthenticatedEmitter {
    pub description: String,
}

pub struct AuthenticatedUser {}

#[async_trait]
impl FromRequestParts<StateType> for AuthenticatedEmitter {
    type Rejection = AppError;

    #[tracing::instrument(skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &StateType,
    ) -> Result<Self, Self::Rejection> {
        let connection = state.lock().await;

        let headers = match HeaderMap::from_request_parts(parts, state).await {
            Ok(headers) => headers,
            Err(_) => return Err(AppError::Status(StatusCode::INTERNAL_SERVER_ERROR)),
        };

        let token = match headers.get("api-token") {
            Some(token) => token
                .to_str()
                .map_err(|_| AppError::Status(StatusCode::BAD_REQUEST))
                .unwrap(),
            None => {
                error!(message = "Missing token header");
                return Err(AppError::Status(StatusCode::UNAUTHORIZED));
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
impl FromRequestParts<StateType> for AuthenticatedUser {
    type Rejection = Response;

    #[tracing::instrument(skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &StateType,
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

        if auth == "Basic YXNkZjp0ZXN0" {
            return Ok(AuthenticatedUser {});
        }

        return Err((StatusCode::UNAUTHORIZED, response_headers).into_response());
    }
}
