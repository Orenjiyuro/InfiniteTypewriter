use infinite_typewriter_core::{
    dry_run_private_corpus_migration, MigrationDryRun, MigrationSource, MigrationTarget,
};

#[tauri::command]
pub fn dry_run_private_corpus_migration_command(
    source: MigrationSource,
    target: MigrationTarget,
    generated_at: String,
) -> Result<MigrationDryRun, String> {
    dry_run_private_corpus_migration(source, target, generated_at)
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use infinite_typewriter_core::{MigrationSource, MigrationTarget};

    use super::dry_run_private_corpus_migration_command;

    #[test]
    fn command_returns_dry_run_without_writing_library_files() {
        let source_root = unique_temp_path("command-source");
        let library_root = unique_temp_path("command-library");
        fs::create_dir_all(source_root.join("breakdowns"))
            .expect("source fixture directory should be created");
        fs::create_dir_all(&library_root).expect("library fixture directory should be created");
        fs::write(
            source_root.join("breakdowns").join("scene-notes.md"),
            "Toy mechanism note for command dry-run.\n",
        )
        .expect("source fixture file should be written");

        let dry_run = dry_run_private_corpus_migration_command(
            MigrationSource::new(source_root.to_string_lossy()),
            MigrationTarget::new(library_root.to_string_lossy()),
            "2026-06-05T00:00:00.000Z".to_string(),
        )
        .expect("command should return dry-run");

        assert_eq!(dry_run.items.len(), 1);
        assert!(!library_root.join("analyses").exists());

        fs::remove_dir_all(source_root).expect("source fixture should clean up");
        fs::remove_dir_all(library_root).expect("library fixture should clean up");
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("infinite-typewriter-migration-{label}-{nanos}"))
    }
}
