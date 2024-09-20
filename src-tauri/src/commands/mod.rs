pub(crate) mod anime;

use crate::db::models::Anime;
use crate::error::AppError;
use sqlx::Row;
use sqlx::{sqlite::SqliteRow, SqlitePool};

fn row_to_anime(row: SqliteRow) -> Anime {
    Anime {
        id: row.get("id"),
        title: row.get("title"),
        description: row.get("description"),
        image_path: row.get("image_path"),
        total_episodes: row.get("total_episodes"),
        release_date: row.get("release_date"),
        end_date: row.get("end_date"),
        status: row.get("status"),
        user_status: row.get("user_status"),
        user_rating: row.get("user_rating"),
    }
}

async fn query_animes(pool: &SqlitePool, query: &str) -> Result<Vec<Anime>, AppError> {
    sqlx::query(query)
        .map(row_to_anime)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
}

async fn query_single_anime(pool: &SqlitePool, query: &str, id: i64) -> Result<Anime, AppError> {
    sqlx::query(query)
        .bind(id)
        .map(row_to_anime)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}
