use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: r#"
                -- Tabla de Animes
                CREATE TABLE IF NOT EXISTS Animes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    description TEXT,
                    image_path TEXT,
                    total_episodes INTEGER,
                    release_date DATE,
                    end_date DATE,
                    status TEXT CHECK(status IN ('En emisión', 'Terminado')),
                    user_status TEXT CHECK(user_status IN ('Visto', 'No visto', 'En progreso', 'Planificado')),
                    user_rating FLOAT CHECK(user_rating >= 0 AND user_rating <= 10),
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );

                -- Tabla de Géneros
                CREATE TABLE IF NOT EXISTS Genres (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE
                );

                -- Tabla de relación Anime-Género
                CREATE TABLE IF NOT EXISTS AnimeGenres (
                    anime_id INTEGER,
                    genre_id INTEGER,
                    PRIMARY KEY (anime_id, genre_id),
                    FOREIGN KEY (anime_id) REFERENCES Animes(id) ON DELETE CASCADE,
                    FOREIGN KEY (genre_id) REFERENCES Genres(id) ON DELETE CASCADE
                );

                -- Tabla de Episodios
                CREATE TABLE IF NOT EXISTS Episodes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    anime_id INTEGER,
                    episode_number INTEGER,
                    title TEXT,
                    duration INTEGER,
                    is_watched BOOLEAN DEFAULT 0,
                    is_favorite BOOLEAN DEFAULT 0,
                    FOREIGN KEY (anime_id) REFERENCES Animes(id) ON DELETE CASCADE
                );

                -- Tabla de Comentarios
                CREATE TABLE IF NOT EXISTS Comments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    anime_id INTEGER,
                    comment TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (anime_id) REFERENCES Animes(id) ON DELETE CASCADE
                );

                -- Índices para mejorar el rendimiento
                CREATE INDEX idx_animes_title ON Animes(title);
                CREATE INDEX idx_animes_user_status ON Animes(user_status);
                CREATE INDEX idx_episodes_anime_id ON Episodes(anime_id);
                CREATE INDEX idx_comments_anime_id ON Comments(anime_id);

                -- Trigger para actualizar el campo updated_at en la tabla Animes
                CREATE TRIGGER update_anime_timestamp AFTER UPDATE ON Animes
                BEGIN
                    UPDATE Animes SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
                END;
            "#,
            kind: MigrationKind::Up,
        },
    ]
}