use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    http::{Request, Response},
    routing::{get, post},
    Router,
};
use dashboards::weight::get_weight;
use data::{get_data, upload_data};
use duckdb::Connection;
use gps::{get_gps_coords, upload_gps_data};
use migration::apply_migrations;
use spa::static_handler;
use tokio::{signal, sync::Mutex};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info, Span};
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;

mod dashboards;
mod data;
mod gps;
mod migration;
mod spa;

type StateType = Arc<Mutex<Connection>>;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    let conn = Arc::new(Mutex::new(Connection::open("./db/db.duckdb")?));

    info!("Opened database connection");

    apply_migrations(conn.clone()).await?;

    let app = Router::new()
        .route("/api/data", post(upload_data))
        .route("/api/data", get(get_data))
        .route("/api/weight", get(get_weight))
        .route("/api/gps", post(upload_gps_data))
        .route("/api/gps", get(get_gps_coords))
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
        .with_state(conn);

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!(message = "Starting server", port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

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
