pub mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::preview_empty_library_manifest,
            commands::migration::dry_run_private_corpus_migration_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running InfiniteTypewriter");
}

#[cfg(test)]
mod tests {
    use infinite_typewriter_core::LibraryRoot;

    use super::commands::preview_empty_library_manifest;

    #[test]
    fn previews_empty_manifest_without_initializing_user_files() {
        let manifest = preview_empty_library_manifest(
            LibraryRoot {
                id: "root-local-demo".to_string(),
                label: "Demo Library".to_string(),
                path: "C:/Users/demo/InfiniteTypewriter".to_string(),
            },
            "2026-06-05T00:00:00.000Z".to_string(),
        );

        assert_eq!(manifest.schema_version, 1);
        assert!(manifest.sources.is_empty());
        assert!(manifest.works.is_empty());
        assert!(manifest.analyses.is_empty());
    }
}
