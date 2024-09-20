mod commands;
mod db;
mod error;

use log::{error, info};
use tauri::{Emitter, Manager};
use commands::anime;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::RotationStrategy;
use tauri_plugin_updater::UpdaterExt;

fn setup_logger(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // create the log plugin as usual, but call split() instead of build()
    let (tauri_plugin_log, _max_level, logger) = tauri_plugin_log::Builder::new()
        .max_file_size(1024 * 1024 * 10) // 10 MB
        .rotation_strategy(RotationStrategy::KeepAll) // keep all logs in the log directory
        .level(log::LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default())
        .level_for("tauri", log::LevelFilter::Warn)
        .level_for("wry", log::LevelFilter::Warn)
        .level_for("tracing", log::LevelFilter::Warn)
        .level_for("anitrack_lib", log::LevelFilter::Info)
        .split(app.handle())?;

    // on debug builds, set up the DevTools plugin and pipe the logger from tauri-plugin-log
    #[cfg(debug_assertions)]
    {
        let mut devtools_builder = tauri_plugin_devtools::Builder::default();
        devtools_builder.attach_logger(logger);
        app.handle().plugin(devtools_builder.init())?;
    }
    // on release builds, only attach the logger from tauri-plugin-log
    #[cfg(not(debug_assertions))]
    {
        tauri_plugin_log::attach_logger(_max_level, logger);
    }

    app.handle().plugin(tauri_plugin_log)?;

    Ok(())
}

async fn check_update(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking for updates");
    let update = app.updater().unwrap().check().await?;
    if let Some(update) = update {
        // The update is available, notify the main window
        info!("Update available: {}", update.version);
        let window = app.get_webview_window("main").unwrap();
        window.emit("update-available", update.version)?;
    }else { 
        info!("No update available");
    }
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .setup(|app| {
            setup_logger(app)?;

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = check_update(app_handle).await {
                    error!("Error checking for updates: {}", e);
                }
            });


            Ok(())
        });
    builder
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(db::init_sql_plugin().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            anime::add_anime,
            anime::delete_anime,
            anime::fetch_animes,
            anime::update_anime,
            anime::save_image,
            anime::get_anime_by_id,
             anime::get_episodes_by_anime_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
