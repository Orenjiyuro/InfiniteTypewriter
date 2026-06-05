pub mod migration;

use infinite_typewriter_core::{create_empty_manifest, LibraryManifest, LibraryRoot};

#[tauri::command]
pub fn preview_empty_library_manifest(root: LibraryRoot, timestamp: String) -> LibraryManifest {
    create_empty_manifest(root, timestamp)
}
