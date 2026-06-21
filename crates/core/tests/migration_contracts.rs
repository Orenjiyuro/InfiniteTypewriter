use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use infinite_typewriter_core::{
    dry_run_private_corpus_migration, MigrationSource, MigrationTarget, MigrationTargetKind,
};

#[test]
fn dry_run_scans_public_safe_toy_fixture_without_returning_file_text() {
    let root = unique_temp_path("dry-run");
    fs::create_dir_all(root.join("breakdowns")).expect("fixture directory should be created");
    fs::write(
        root.join("breakdowns").join("scene-notes.md"),
        "Toy mechanism note: a visible limit creates pressure.\n",
    )
    .expect("fixture file should be written");

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("toy dry-run should scan");

    assert_eq!(dry_run.kind, "migration-dry-run");
    assert_eq!(dry_run.items.len(), 1);
    assert_eq!(dry_run.items[0].relative_path, "breakdowns/scene-notes.md");
    assert_eq!(dry_run.items[0].target.kind, MigrationTargetKind::Analysis);
    assert!(dry_run.items[0].content_hash.starts_with("fnv1a64:"));
    assert_eq!(dry_run.report.total_files, 1);
    assert_eq!(dry_run.report.migratable_files, 1);
    assert_eq!(dry_run.report.blocked_files, 0);

    let serialized = serde_json::to_string(&dry_run).expect("dry-run should serialize");
    assert!(!serialized.contains("visible limit creates pressure"));
    assert!(!serialized.contains("blockedReason"));

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[test]
fn dry_run_rejects_include_paths_outside_selected_source_root() {
    let root = unique_temp_path("path-safe");
    let outside = unique_temp_path("outside");
    fs::create_dir_all(&root).expect("source root should be created");
    fs::write(&outside, "outside file should never be scanned").expect("outside file exists");

    let source = MigrationSource::new(root.to_string_lossy()).with_include_paths(["../outside"]);
    let error = dry_run_private_corpus_migration(
        source,
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect_err("path traversal should be rejected");

    assert!(error.to_string().contains("outside selected source root"));

    fs::remove_dir_all(root).expect("source root should clean up");
    fs::remove_file(outside).expect("outside file should clean up");
}

#[test]
fn dry_run_rejects_windows_drive_prefix_include_paths() {
    let root = unique_temp_path("windows-prefix");
    fs::create_dir_all(&root).expect("source root should be created");

    let source = MigrationSource::new(root.to_string_lossy()).with_include_paths(["C:outside"]);
    let error = dry_run_private_corpus_migration(
        source,
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect_err("drive-prefixed include path should be rejected");

    assert!(error.to_string().contains("outside selected source root"));

    fs::remove_dir_all(root).expect("source root should clean up");
}

#[test]
fn content_hashes_are_stable_for_same_relative_path_and_bytes() {
    let left = write_hash_fixture("hash-left");
    let right = write_hash_fixture("hash-right");

    let left_dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(left.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("left fixture should scan");
    let right_dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(right.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("right fixture should scan");

    assert_eq!(
        left_dry_run.items[0].content_hash,
        right_dry_run.items[0].content_hash
    );
    assert_eq!(
        left_dry_run.report.source_tree_hash,
        right_dry_run.report.source_tree_hash
    );

    fs::remove_dir_all(left).expect("left fixture should clean up");
    fs::remove_dir_all(right).expect("right fixture should clean up");
}

#[test]
fn content_hashes_include_chunk_boundaries_between_path_and_file_bytes() {
    let root = unique_temp_path("hash-boundary");
    fs::create_dir_all(&root).expect("fixture directory should be created");
    fs::write(root.join("a"), "bc").expect("first file should be written");
    fs::write(root.join("ab"), "c").expect("second file should be written");

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    assert_eq!(dry_run.items[0].relative_path, "a");
    assert_eq!(dry_run.items[1].relative_path, "ab");
    assert_ne!(dry_run.items[0].content_hash, dry_run.items[1].content_hash);

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[test]
fn target_paths_preserve_relative_directories_to_avoid_collisions() {
    let root = unique_temp_path("target-paths");
    fs::create_dir_all(root.join("first")).expect("first directory should be created");
    fs::create_dir_all(root.join("second")).expect("second directory should be created");
    fs::write(root.join("first").join("notes.md"), "Toy first note.\n")
        .expect("first note should be written");
    fs::write(root.join("second").join("notes.md"), "Toy second note.\n")
        .expect("second note should be written");

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    let target_paths = dry_run
        .items
        .iter()
        .map(|item| item.target.library_relative_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        target_paths,
        ["analyses/first/notes.json", "analyses/second/notes.json"]
    );

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[test]
fn target_path_collisions_are_blocked_in_the_dry_run_report() {
    let root = unique_temp_path("target-collisions");
    fs::create_dir_all(root.join("notes")).expect("notes directory should be created");
    fs::write(
        root.join("notes").join("mechanism.md"),
        "Toy markdown note.\n",
    )
    .expect("markdown note should be written");
    fs::write(root.join("notes").join("mechanism.json"), "{}\n")
        .expect("json note should be written");

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    assert_eq!(dry_run.report.total_files, 2);
    assert_eq!(dry_run.report.migratable_files, 0);
    assert_eq!(dry_run.report.blocked_files, 2);
    assert!(dry_run.items.iter().all(|item| item
        .blocked_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("target path collision"))));

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[test]
fn overlapping_include_paths_do_not_duplicate_source_items() {
    let root = unique_temp_path("overlap");
    fs::create_dir_all(root.join("notes")).expect("notes directory should be created");
    fs::write(
        root.join("notes").join("mechanism.md"),
        "Toy markdown note.\n",
    )
    .expect("markdown note should be written");

    let source = MigrationSource::new(root.to_string_lossy())
        .with_include_paths(["notes", "notes/mechanism.md"]);
    let dry_run = dry_run_private_corpus_migration(
        source,
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    assert_eq!(dry_run.items.len(), 1);
    assert_eq!(dry_run.items[0].relative_path, "notes/mechanism.md");

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[cfg(unix)]
#[test]
fn symlink_items_are_blocked_instead_of_migrated() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_path("symlink");
    fs::create_dir_all(&root).expect("fixture directory should be created");
    fs::write(root.join("real.md"), "Toy mechanism note.\n")
        .expect("real fixture file should be written");
    symlink(root.join("real.md"), root.join("linked.md")).expect("symlink should be created");

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    let linked_item = dry_run
        .items
        .iter()
        .find(|item| item.relative_path == "linked.md")
        .expect("linked item should be reported");

    assert_eq!(
        linked_item.blocked_reason.as_deref(),
        Some("symbolic links are not migrated")
    );

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[cfg(windows)]
#[test]
fn symlink_items_are_blocked_instead_of_migrated() {
    use std::os::windows::fs::symlink_file;

    let root = unique_temp_path("symlink");
    fs::create_dir_all(&root).expect("fixture directory should be created");
    fs::write(root.join("real.md"), "Toy mechanism note.\n")
        .expect("real fixture file should be written");

    if symlink_file(root.join("real.md"), root.join("linked.md")).is_err() {
        fs::remove_dir_all(root).expect("fixture directory should clean up");
        return;
    }

    let dry_run = dry_run_private_corpus_migration(
        MigrationSource::new(root.to_string_lossy()),
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    let linked_item = dry_run
        .items
        .iter()
        .find(|item| item.relative_path == "linked.md")
        .expect("linked item should be reported");

    assert_eq!(
        linked_item.blocked_reason.as_deref(),
        Some("symbolic links are not migrated")
    );

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[cfg(unix)]
#[test]
fn explicitly_included_symlink_items_are_blocked_instead_of_followed() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_path("include-symlink");
    fs::create_dir_all(&root).expect("fixture directory should be created");
    fs::write(root.join("real.md"), "Toy mechanism note.\n")
        .expect("real fixture file should be written");
    symlink(root.join("real.md"), root.join("linked.md")).expect("symlink should be created");

    let source = MigrationSource::new(root.to_string_lossy()).with_include_paths(["linked.md"]);
    let dry_run = dry_run_private_corpus_migration(
        source,
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    assert_eq!(dry_run.items.len(), 1);
    assert_eq!(dry_run.items[0].relative_path, "linked.md");
    assert_eq!(
        dry_run.items[0].blocked_reason.as_deref(),
        Some("symbolic links are not migrated")
    );

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

#[cfg(windows)]
#[test]
fn explicitly_included_symlink_items_are_blocked_instead_of_followed() {
    use std::os::windows::fs::symlink_file;

    let root = unique_temp_path("include-symlink");
    fs::create_dir_all(&root).expect("fixture directory should be created");
    fs::write(root.join("real.md"), "Toy mechanism note.\n")
        .expect("real fixture file should be written");

    if symlink_file(root.join("real.md"), root.join("linked.md")).is_err() {
        fs::remove_dir_all(root).expect("fixture directory should clean up");
        return;
    }

    let source = MigrationSource::new(root.to_string_lossy()).with_include_paths(["linked.md"]);
    let dry_run = dry_run_private_corpus_migration(
        source,
        MigrationTarget::new("C:/Users/demo/InfiniteTypewriter"),
        "2026-06-05T00:00:00.000Z",
    )
    .expect("fixture should scan");

    assert_eq!(dry_run.items.len(), 1);
    assert_eq!(dry_run.items[0].relative_path, "linked.md");
    assert_eq!(
        dry_run.items[0].blocked_reason.as_deref(),
        Some("symbolic links are not migrated")
    );

    fs::remove_dir_all(root).expect("fixture directory should clean up");
}

fn write_hash_fixture(label: &str) -> PathBuf {
    let root = unique_temp_path(label);
    fs::create_dir_all(root.join("notes")).expect("fixture directory should be created");
    fs::write(
        root.join("notes").join("mechanism.md"),
        "Toy mechanism note with stable bytes.\n",
    )
    .expect("fixture file should be written");
    root
}

fn unique_temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("infinite-typewriter-migration-{label}-{nanos}"))
}
