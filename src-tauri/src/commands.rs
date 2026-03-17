use rusqlite::Connection;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

pub type DbState = Arc<Mutex<Connection>>;

#[derive(Debug, Serialize, Clone)]
pub struct Clip {
    pub id: i64,
    pub content: String,
    pub preview: String,
    pub created_at: String,
    pub pinned: bool,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub sort_order: i64,
}

#[tauri::command]
pub fn get_clips(
    db: State<'_, DbState>,
    category_id: Option<i64>,
    pinned_only: bool,
    search: Option<String>,
) -> Result<Vec<Clip>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut sql = String::from(
        "SELECT c.id, c.content, c.preview, c.created_at, c.pinned, c.category_id, cat.name as category_name
         FROM clips c
         LEFT JOIN categories cat ON c.category_id = cat.id
         WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if pinned_only {
        sql.push_str(" AND c.pinned = 1");
    }

    if let Some(cat_id) = category_id {
        sql.push_str(" AND c.category_id = ?");
        params.push(Box::new(cat_id));
    }

    if let Some(ref query) = search {
        if !query.trim().is_empty() {
            sql.push_str(" AND c.content LIKE ?");
            params.push(Box::new(format!("%{}%", query.trim())));
        }
    }

    sql.push_str(" ORDER BY c.pinned DESC, c.created_at DESC LIMIT 500");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let clips = stmt
        .query_map(param_refs.as_slice(), |row| {
            Ok(Clip {
                id: row.get(0)?,
                content: row.get(1)?,
                preview: row.get(2)?,
                created_at: row.get(3)?,
                pinned: row.get::<_, i64>(4)? != 0,
                category_id: row.get(5)?,
                category_name: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(clips)
}

#[tauri::command]
pub fn delete_clip(db: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM clips WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_pin(db: State<'_, DbState>, id: i64) -> Result<bool, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE clips SET pinned = CASE WHEN pinned = 0 THEN 1 ELSE 0 END WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;

    let pinned: bool = conn
        .query_row("SELECT pinned FROM clips WHERE id = ?1", [id], |row| {
            Ok(row.get::<_, i64>(0)? != 0)
        })
        .map_err(|e| e.to_string())?;

    Ok(pinned)
}

#[tauri::command]
pub fn set_clip_category(
    db: State<'_, DbState>,
    clip_id: i64,
    category_id: Option<i64>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE clips SET category_id = ?1 WHERE id = ?2",
        rusqlite::params![category_id, clip_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_categories(db: State<'_, DbState>) -> Result<Vec<Category>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, icon, sort_order FROM categories ORDER BY sort_order, name")
        .map_err(|e| e.to_string())?;

    let categories = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                sort_order: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(categories)
}

#[tauri::command]
pub fn add_category(
    db: State<'_, DbState>,
    name: String,
    icon: String,
) -> Result<Category, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO categories (name, icon) VALUES (?1, ?2)",
        [&name, &icon],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    Ok(Category {
        id,
        name,
        icon,
        sort_order: 0,
    })
}

#[tauri::command]
pub fn delete_category(db: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE clips SET category_id = NULL WHERE category_id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM categories WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn clear_history(db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM clips WHERE pinned = 0", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn copy_clip_to_clipboard(
    db: State<'_, DbState>,
    id: i64,
) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let content: String = conn
        .query_row("SELECT content FROM clips WHERE id = ?1", [id], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;
    Ok(content)
}

#[tauri::command]
pub fn hide_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}
