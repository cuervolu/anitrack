mod migrations;
pub(crate) mod models;

use crate::error::AppError;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::Builder as SqlBuilder;

pub fn init_sql_plugin() -> SqlBuilder {
    let migrations = migrations::get_migrations();
    SqlBuilder::default().add_migrations("sqlite:animetrack.db", migrations)
}

pub async fn get_db(app_handle: &AppHandle) -> Result<SqlitePool, AppError> {
    let path = app_handle.path().app_config_dir()?;
    let db_path = path.join("animetrack.db");
    SqlitePool::connect(db_path.to_str().unwrap())
        .await
        .map_err(AppError::Database)
}
