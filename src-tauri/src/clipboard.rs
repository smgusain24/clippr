use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "macos")]
fn get_clipboard_text() -> Option<(String, i64)> {
    use objc2_app_kit::NSPasteboard;
    use objc2_foundation::NSString;

    let pasteboard = NSPasteboard::generalPasteboard();
    let change_count = pasteboard.changeCount() as i64;

    let ns_string_type = NSString::from_str("public.utf8-plain-text");
    if let Some(content) = pasteboard.stringForType(&ns_string_type) {
        let text = content.to_string();
        if !text.trim().is_empty() {
            return Some((text, change_count));
        }
    }
    Some((String::new(), change_count))
}

#[cfg(not(target_os = "macos"))]
fn get_clipboard_text() -> Option<(String, i64)> {
    None
}

fn make_preview(content: &str) -> String {
    let trimmed = content.trim();
    if trimmed.len() <= 100 {
        trimmed.to_string()
    } else {
        format!("{}…", &trimmed[..100])
    }
}

pub fn start_clipboard_monitor(app_handle: AppHandle, db: Arc<Mutex<Connection>>) {
    thread::spawn(move || {
        let mut last_change_count: i64 = -1;
        let mut last_content = String::new();

        // Get initial change count without storing content
        if let Some((_, count)) = get_clipboard_text() {
            last_change_count = count;
        }

        loop {
            thread::sleep(Duration::from_millis(500));

            if let Some((text, change_count)) = get_clipboard_text() {
                if change_count != last_change_count && !text.trim().is_empty() && text != last_content {
                    last_change_count = change_count;
                    last_content = text.clone();

                    let preview = make_preview(&text);

                    if let Ok(conn) = db.lock() {
                        let result = conn.execute(
                            "INSERT INTO clips (content, preview) VALUES (?1, ?2)",
                            [&text, &preview],
                        );

                        if result.is_ok() {
                            let _ = app_handle.emit("clip-added", ());
                        }
                    }
                } else if change_count != last_change_count {
                    last_change_count = change_count;
                }
            }
        }
    });
}
