use std::fs;
use std::path::PathBuf;
use log::info;
use crate::commands::{query_animes, query_single_anime};
use crate::db::get_db;
use crate::db::models::{Anime, Episode, NewAnime};
use crate::error::AppError;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn fetch_animes(app_handle: AppHandle) -> Result<Vec<Anime>, AppError> {
    let pool = get_db(&app_handle).await?;
    query_animes(&pool, "SELECT * FROM Animes").await
}

#[tauri::command]
pub async fn add_anime(app_handle: AppHandle, anime: NewAnime) -> Result<Anime, AppError> {
    let pool = get_db(&app_handle).await?;
    let inserted_id = sqlx::query(
        "INSERT INTO Animes (title, description, image_path, total_episodes, release_date, end_date, status, user_status, user_rating)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(&anime.title)
        .bind(&anime.description)
        .bind(&anime.image_path)
        .bind(anime.total_episodes)
        .bind(&anime.release_date)
        .bind(&anime.end_date)
        .bind(&anime.status)
        .bind(&anime.user_status)
        .bind(anime.user_rating)
        .execute(&pool)
        .await
        .map_err(AppError::Database)?
        .last_insert_rowid();

    query_single_anime(&pool, "SELECT * FROM Animes WHERE id = ?", inserted_id).await
}

#[tauri::command]
pub async fn update_anime(app_handle: AppHandle, id: i64, anime: NewAnime) -> Result<Anime, AppError> {
    info!("Updating anime with id: {}", id);
    let pool = get_db(&app_handle).await?;
    sqlx::query(
        "UPDATE Animes
         SET title = ?, description = ?, image_path = ?, total_episodes = ?, release_date = ?, end_date = ?, status = ?, user_status = ?, user_rating = ?
         WHERE id = ?"
    )
        .bind(&anime.title)
        .bind(&anime.description)
        .bind(&anime.image_path)
        .bind(anime.total_episodes)
        .bind(&anime.release_date)
        .bind(&anime.end_date)
        .bind(&anime.status)
        .bind(&anime.user_status)
        .bind(anime.user_rating)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::Database)?;

    query_single_anime(&pool, "SELECT * FROM Animes WHERE id = ?", id).await
}

#[tauri::command]
pub async fn delete_anime(app_handle: AppHandle, id: i64) -> Result<(), AppError> {
    let pool = get_db(&app_handle).await?;
    sqlx::query("DELETE FROM Animes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::Database)?;

    Ok(())
}

#[tauri::command]
pub async fn get_anime_by_id(app_handle: AppHandle, id: i64) -> Result<Anime, AppError> {
    let pool = get_db(&app_handle).await?;
    query_single_anime(&pool, "SELECT * FROM Animes WHERE id = ?", id).await
}

#[tauri::command]
pub async fn get_episodes_by_anime_id(app_handle: AppHandle, anime_id: i64) -> Result<Vec<Episode>, AppError> {
    let pool = get_db(&app_handle).await?;
    sqlx::query_as::<_, Episode>("SELECT * FROM Episodes WHERE anime_id = ?")
        .bind(anime_id)
        .fetch_all(&pool)
        .await
        .map_err(AppError::Database)
}


#[tauri::command]
pub async fn save_image(app_handle: AppHandle, source_path: String) -> Result<String, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir().map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let images_dir = app_data_dir.join("images");
    fs::create_dir_all(&images_dir)
        .map_err(|e| format!("Failed to create images directory: {}", e))?;

    let source_path = PathBuf::from(source_path);
    let file_name = source_path
        .file_name()
        .ok_or_else(|| "Invalid source path".to_string())?;
    let destination_path = images_dir.join(file_name);

    fs::copy(&source_path, &destination_path)
        .map_err(|e| format!("Failed to copy image: {}", e))?;

    Ok(destination_path.to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?
        .to_string())
}
