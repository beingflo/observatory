use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use data::{get_data, upload_data};
use duckdb::Connection;
use gps::{get_gps_coords, upload_gps_data};
use tokio::sync::Mutex;
use tracing::info;

mod data;
mod gps;

type StateType = Arc<Mutex<Connection>>;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let conn = Arc::new(Mutex::new(Connection::open("./db.duckdb")?));

    conn.lock().await.execute_batch(
        r"CREATE TABLE IF NOT EXISTS data (
            timestamp TIMESTAMP NOT NULL,
            bucket TEXT NOT NULL,
            payload JSON NOT NULL
          );
        ",
    )?;

    info!(message = "Created table");

    let app = Router::new()
        .route("/", post(upload_data))
        .route("/", get(get_data))
        .route("/gps", post(upload_gps_data))
        .route("/gps", get(get_gps_coords))
        .with_state(conn);

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!(message = "Starting server", port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
