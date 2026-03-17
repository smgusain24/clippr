mod clipboard;
mod commands;
mod db;
mod tray;

use commands::DbState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("clippr.db");
            let conn = db::init_db(&db_path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let _ = db::cleanup_old_clips(&conn, 7);

            let db: DbState = Arc::new(Mutex::new(conn));
            app.manage(db.clone());

            clipboard::start_clipboard_monitor(app.handle().clone(), db.clone());

            let cleanup_db = db.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(3600));
                if let Ok(conn) = cleanup_db.lock() {
                    let _ = db::cleanup_old_clips(&conn, 7);
                }
            });

            tray::setup_tray(app.handle())?;

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clips,
            commands::delete_clip,
            commands::toggle_pin,
            commands::set_clip_category,
            commands::get_categories,
            commands::add_category,
            commands::delete_category,
            commands::clear_history,
            commands::copy_clip_to_clipboard,
            commands::hide_window,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if !tray::should_skip_focus_loss() {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
