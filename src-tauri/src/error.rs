use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to get app config directory")]
    AppDataDir,

    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to open browser")]
    ShellError(#[from] tauri_plugin_shell::Error),

}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
