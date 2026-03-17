use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

pub fn init_db(db_path: &Path) -> SqlResult<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            icon TEXT NOT NULL DEFAULT '📁',
            sort_order INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            preview TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            pinned INTEGER NOT NULL DEFAULT 0,
            category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_clips_created_at ON clips(created_at);
        CREATE INDEX IF NOT EXISTS idx_clips_category ON clips(category_id);
        CREATE INDEX IF NOT EXISTS idx_clips_pinned ON clips(pinned);
        ",
    )?;

    Ok(conn)
}

pub fn cleanup_old_clips(conn: &Connection, retention_days: i64) -> SqlResult<usize> {
    conn.execute(
        "DELETE FROM clips WHERE pinned = 0 AND created_at < datetime('now', ?1)",
        [format!("-{} days", retention_days)],
    )
}
