use std::sync::atomic::{AtomicI64, Ordering};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

static LAST_TOGGLE_MS: AtomicI64 = AtomicI64::new(0);

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit Clippr", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let icon = Image::from_bytes(include_bytes!("../icons/tray-icon@2x.png"))?;

    TrayIconBuilder::new()
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Clippr")
        .on_menu_event(|app, event| {
            if event.id.as_ref() == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button_state,
                rect,
                ..
            } = event
            {
                if button_state == tauri::tray::MouseButtonState::Up {
                    let app = tray.app_handle();
                    let pos = rect.position.to_physical::<f64>(1.0);
                    let size = rect.size.to_physical::<f64>(1.0);
                    toggle_window(app, Some((pos.x, pos.y, size.width)));
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn toggle_window(app: &AppHandle, tray_rect: Option<(f64, f64, f64)>) {
    LAST_TOGGLE_MS.store(now_ms(), Ordering::Relaxed);

    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            if let Some((tray_x, tray_y, tray_w)) = tray_rect {
                position_window_below_tray(&window, tray_x, tray_y, tray_w);
            }
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

pub fn should_skip_focus_loss() -> bool {
    let last = LAST_TOGGLE_MS.load(Ordering::Relaxed);
    (now_ms() - last) < 500
}

fn position_window_below_tray(
    window: &tauri::WebviewWindow,
    tray_x: f64,
    tray_y: f64,
    tray_w: f64,
) {
    let scale = window.scale_factor().unwrap_or(2.0);

    // Get the window's current size (user may have resized)
    let win_width = window
        .outer_size()
        .map(|s| s.width as f64 / scale)
        .unwrap_or(420.0);

    let menu_bar_height = 25.0;

    let tray_center_x = (tray_x / scale) + (tray_w / scale / 2.0);
    let x = tray_center_x - (win_width / 2.0);
    let y = (tray_y / scale) + menu_bar_height;

    // Clamp to screen edges
    let x = if x < 8.0 { 8.0 } else { x };

    let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)));
}
