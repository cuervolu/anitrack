use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub total_episodes: Option<i32>,
    pub release_date: Option<String>,
    pub end_date: Option<String>,
    pub status: String,
    pub user_status: String,
    pub user_rating: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Episode {
    pub id: i64,
    pub anime_id: i64,
    pub episode_number: i32,
    pub title: Option<String>,
    pub duration: Option<i32>,
    pub is_watched: bool,
    pub is_favorite: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAnime {
    pub title: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub total_episodes: Option<i32>,
    pub release_date: Option<String>,
    pub end_date: Option<String>,
    pub status: String,
    pub user_status: String,
    pub user_rating: Option<f32>,
}