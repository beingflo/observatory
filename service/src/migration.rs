use std::sync::Arc;

use duckdb::Connection;
use tokio::sync::Mutex;
use tracing::info;

#[tracing::instrument(skip_all)]
pub async fn apply_migrations(conn: Arc<Mutex<Connection>>) -> Result<(), duckdb::Error> {
    let conn = conn.lock().await;

    conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS timeseries (
            timestamp TIMESTAMPTZ NOT NULL,
            bucket TEXT NOT NULL,
            payload JSON NOT NULL
          );
        ",
    )?;

    conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS emitters (
            created_at TIMESTAMPTZ NOT NULL,
            expires_at TIMESTAMPTZ,
            token TEXT NOT NULL,
            description TEXT NOT NULL
          );
        ",
    )?;

    info!(message = "Applied migrations");

    Ok(())
}
