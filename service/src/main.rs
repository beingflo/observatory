use std::{env, process::abort, sync::Arc, time::Duration};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    routing::{delete, get, post},
    Router,
};
use dashboards::{observatory::get_observatory_info, weight::get_weight};
use data::{delete_data, get_data, upload_data, upload_data_url_only};
use duckdb::Connection;
use emitters::{add_emitter, delete_emitter, get_emitters};
use error::AppError;
use gps::{get_gps_coords, upload_gps_data};
use migration::apply_migrations;
use spa::static_handler;
use tokio::{signal, sync::Mutex};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{error, info, warn, Span};
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;

mod auth;
mod dashboards;
mod data;
mod emitters;
mod error;
mod gps;
mod migration;
mod spa;
mod utils;

#[derive(Clone)]
struct AppState {
    connection: Arc<Mutex<Connection>>,
    admin_auth: String,
}

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    match dotenvy::dotenv() {
        Ok(_) => info!("Loaded .env file"),
        Err(_) => warn!("Failed to load .env file"),
    };

    let Some((_, basic_auth)) = env::vars().find(|v| v.0.eq("ADMIN_BASIC_AUTH")) else {
        error!("Admin auth credentials not in environment");
        abort();
    };
    info!("Found ADMIN_BASIC_AUTH in environment");

    let conn = Arc::new(Mutex::new(Connection::open("./db/db.duckdb")?));

    info!("Opened database connection");

    apply_migrations(conn.clone()).await?;

    let app = Router::new()
        .route("/api/data", post(upload_data))
        .route("/api/data/:emitter/:bucket", post(upload_data_url_only))
        .route("/api/data", get(get_data))
        .route("/api/data", delete(delete_data))
        .route("/api/weight", get(get_weight))
        .route("/api/observatory", get(get_observatory_info))
        .route("/api/gps/:emitter/:bucket", post(upload_gps_data))
        .route("/api/gps/:bucket", get(get_gps_coords))
        .route("/api/emitter", get(get_emitters))
        .route("/api/emitter", post(add_emitter))
        .route("/api/emitter", delete(delete_emitter))
        .fallback(static_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    let request_id = Uuid::new_v4().to_string();
                    tracing::info_span!("http-request", %request_id)
                })
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::info!("request: {} {}", request.method(), request.uri().path())
                })
                .on_response(
                    |response: &Response<Body>, latency: Duration, _span: &Span| {
                        tracing::info!("response: {} {:?}", response.status(), latency)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("error: {}", error)
                    },
                ),
        )
        .with_state(AppState {
            connection: conn,
            admin_auth: basic_auth,
        });

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| {
            error!(message = "Failed to create TCP listener", error=%e);
            AppError::Status(StatusCode::SERVICE_UNAVAILABLE)
        })?;

    info!(message = "Starting server", port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!(message = "Failed to start server", error=%e);
            AppError::Status(StatusCode::SERVICE_UNAVAILABLE)
        })?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl+C received, shutting down")
        },
        _ = terminate => {
            info!("SIGTERM received, shutting down")
        },
    }
}
