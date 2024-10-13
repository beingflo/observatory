use std::sync::Arc;

use duckdb::Connection;
use tokio::sync::Mutex;
use tracing::info;

#[tracing::instrument(skip_all)]
pub async fn apply_migrations(conn: Arc<Mutex<Connection>>) -> Result<(), duckdb::Error> {
    conn.lock().await.execute_batch(
        r"CREATE TABLE IF NOT EXISTS data (
            timestamp TIMESTAMPTZ NOT NULL,
            bucket TEXT NOT NULL,
            payload JSON NOT NULL
          );
        ",
    )?;

    info!(message = "Applied migrations");

    Ok(())
}
