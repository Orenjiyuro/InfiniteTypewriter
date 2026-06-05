use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::{params, Connection};

use crate::model::{create_empty_manifest, LibraryManifest, LibraryRoot};

const LOCAL_DIRECTORIES: &[&str] = &["sources", "analyses", "works", "runs", "recipes", "indexes"];

#[derive(Debug)]
pub enum LibraryError {
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for LibraryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "library file operation failed: {error}"),
            Self::Sqlite(error) => write!(formatter, "library SQLite operation failed: {error}"),
            Self::Json(error) => {
                write!(formatter, "library manifest serialization failed: {error}")
            }
        }
    }
}

impl std::error::Error for LibraryError {}

impl From<std::io::Error> for LibraryError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for LibraryError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<serde_json::Error> for LibraryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub fn initialize_empty_library(
    root_path: impl AsRef<Path>,
    root_id: impl Into<String>,
    label: impl Into<String>,
    timestamp: impl Into<String>,
) -> Result<LibraryManifest, LibraryError> {
    let root_path = root_path.as_ref();
    fs::create_dir_all(root_path)?;

    for directory in LOCAL_DIRECTORIES {
        fs::create_dir_all(root_path.join(directory))?;
    }

    let root = LibraryRoot {
        id: root_id.into(),
        label: label.into(),
        path: root_path.to_string_lossy().to_string(),
    };
    let manifest = create_empty_manifest(root, timestamp);

    write_manifest(root_path, &manifest)?;
    initialize_catalog(root_path.join("indexes").join("catalog.sqlite"), &manifest)?;

    Ok(manifest)
}

fn write_manifest(root_path: &Path, manifest: &LibraryManifest) -> Result<(), LibraryError> {
    let manifest_json = serde_json::to_string_pretty(manifest)?;
    fs::write(root_path.join("manifest.json"), manifest_json)?;
    Ok(())
}

fn initialize_catalog(
    catalog_path: PathBuf,
    manifest: &LibraryManifest,
) -> Result<(), LibraryError> {
    let connection = Connection::open(catalog_path)?;
    connection.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS library_manifest (
            id TEXT PRIMARY KEY,
            label TEXT NOT NULL,
            schema_version INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sources (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS works (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS analyses (
            id TEXT PRIMARY KEY,
            source_id TEXT NOT NULL,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    connection.execute(
        r#"
        INSERT INTO library_manifest (id, label, schema_version, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(id) DO UPDATE SET
            label = excluded.label,
            schema_version = excluded.schema_version,
            updated_at = excluded.updated_at
        "#,
        params![
            manifest.root.id,
            manifest.root.label,
            manifest.schema_version,
            manifest.created_at,
            manifest.updated_at
        ],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use rusqlite::Connection;

    use super::initialize_empty_library;

    #[test]
    fn initializes_empty_library_manifest_and_directories() {
        let root_path = unique_temp_path("manifest");

        let manifest = initialize_empty_library(
            &root_path,
            "root-local-demo",
            "Demo Library",
            "2026-06-05T00:00:00.000Z",
        )
        .expect("empty library should initialize");

        assert_eq!(manifest.schema_version, 1);
        assert_eq!(manifest.root.id, "root-local-demo");
        assert!(manifest.sources.is_empty());
        assert!(manifest.works.is_empty());
        assert!(manifest.analyses.is_empty());

        for directory in ["sources", "analyses", "works", "runs", "recipes", "indexes"] {
            assert!(
                root_path.join(directory).is_dir(),
                "{directory} should exist"
            );
        }

        let manifest_json = fs::read_to_string(root_path.join("manifest.json"))
            .expect("manifest should be written");
        assert!(manifest_json.contains("\"schemaVersion\": 1"));

        fs::remove_dir_all(root_path).expect("test library should clean up");
    }

    #[test]
    fn initializes_sqlite_catalog_for_empty_library() {
        let root_path = unique_temp_path("catalog");

        initialize_empty_library(
            &root_path,
            "root-local-demo",
            "Demo Library",
            "2026-06-05T00:00:00.000Z",
        )
        .expect("empty library should initialize");

        let connection = Connection::open(root_path.join("indexes").join("catalog.sqlite"))
            .expect("catalog opens");
        let table_count: u32 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('library_manifest', 'sources', 'works', 'analyses')",
                [],
                |row| row.get(0),
            )
            .expect("tables should be queryable");
        let manifest_count: u32 = connection
            .query_row("SELECT COUNT(*) FROM library_manifest", [], |row| {
                row.get(0)
            })
            .expect("manifest row should be queryable");

        assert_eq!(table_count, 4);
        assert_eq!(manifest_count, 1);

        drop(connection);
        fs::remove_dir_all(root_path).expect("test library should clean up");
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("infinite-typewriter-{label}-{nanos}"))
    }
}
