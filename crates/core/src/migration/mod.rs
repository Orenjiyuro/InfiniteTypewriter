use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::model::IsoDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationSource {
    pub root_path: String,
    pub include_paths: Vec<String>,
}

impl MigrationSource {
    pub fn new(root_path: impl Into<String>) -> Self {
        Self {
            root_path: root_path.into(),
            include_paths: Vec::new(),
        }
    }

    pub fn with_include_paths<I, S>(mut self, include_paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.include_paths = include_paths.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationTarget {
    pub library_root_path: String,
}

impl MigrationTarget {
    pub fn new(library_root_path: impl Into<String>) -> Self {
        Self {
            library_root_path: library_root_path.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationDryRun {
    pub kind: String,
    pub generated_at: IsoDateTime,
    pub source: MigrationSource,
    pub target: MigrationTarget,
    pub items: Vec<MigrationItem>,
    pub report: MigrationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationItem {
    pub relative_path: String,
    pub content_hash: String,
    pub target: MigrationItemTarget,
    pub status: MigrationItemStatus,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationItemTarget {
    pub kind: MigrationTargetKind,
    pub library_relative_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationTargetKind {
    Source,
    Analysis,
    Work,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationItemStatus {
    Migratable,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationReport {
    pub kind: String,
    pub generated_at: IsoDateTime,
    pub total_files: usize,
    pub migratable_files: usize,
    pub blocked_files: usize,
    pub source_tree_hash: String,
}

#[derive(Debug)]
pub enum MigrationError {
    Io(std::io::Error),
    SourceRootNotDirectory(PathBuf),
    UnsafePath(String),
    NonUtf8Path(PathBuf),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "migration file operation failed: {error}"),
            Self::SourceRootNotDirectory(path) => {
                write!(
                    formatter,
                    "migration source root is not a directory: {}",
                    path.display()
                )
            }
            Self::UnsafePath(path) => {
                write!(
                    formatter,
                    "migration path is outside selected source root: {path}"
                )
            }
            Self::NonUtf8Path(path) => {
                write!(
                    formatter,
                    "migration path is not valid UTF-8: {}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for MigrationError {}

impl From<std::io::Error> for MigrationError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn dry_run_private_corpus_migration(
    source: MigrationSource,
    target: MigrationTarget,
    generated_at: impl Into<IsoDateTime>,
) -> Result<MigrationDryRun, MigrationError> {
    let generated_at = generated_at.into();
    let root_path = PathBuf::from(&source.root_path);
    let canonical_root = fs::canonicalize(&root_path)?;

    if !canonical_root.is_dir() {
        return Err(MigrationError::SourceRootNotDirectory(root_path));
    }

    let mut items = Vec::new();
    if source.include_paths.is_empty() {
        scan_path(&canonical_root, &canonical_root, &mut items)?;
    } else {
        for include_path in &source.include_paths {
            reject_unsafe_include_path(include_path)?;
            let selected_path = canonical_root.join(include_path);
            let canonical_selected = fs::canonicalize(&selected_path)?;
            ensure_inside_root(&canonical_root, &canonical_selected, include_path)?;
            scan_path(&canonical_root, &canonical_selected, &mut items)?;
        }
    }

    items.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    deduplicate_source_items(&mut items);
    mark_target_path_collisions(&mut items);
    let migratable_files = items
        .iter()
        .filter(|item| item.status == MigrationItemStatus::Migratable)
        .count();
    let blocked_files = items.len() - migratable_files;
    let source_tree_hash = source_tree_hash(&items);

    Ok(MigrationDryRun {
        kind: "migration-dry-run".to_string(),
        generated_at: generated_at.clone(),
        source,
        target,
        report: MigrationReport {
            kind: "migration-report".to_string(),
            generated_at,
            total_files: items.len(),
            migratable_files,
            blocked_files,
            source_tree_hash,
        },
        items,
    })
}

fn scan_path(
    root: &Path,
    path: &Path,
    items: &mut Vec<MigrationItem>,
) -> Result<(), MigrationError> {
    ensure_inside_root(root, path, &path.to_string_lossy())?;
    let metadata = fs::symlink_metadata(path)?;

    if metadata.file_type().is_symlink() {
        items.push(blocked_item(root, path, "symbolic links are not migrated")?);
        return Ok(());
    }

    if metadata.is_dir() {
        let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            scan_path(root, &entry.path(), items)?;
        }
        return Ok(());
    }

    if metadata.is_file() {
        items.push(file_item(root, path)?);
    }

    Ok(())
}

fn file_item(root: &Path, path: &Path) -> Result<MigrationItem, MigrationError> {
    let relative_path = relative_path(root, path)?;
    let bytes = fs::read(path)?;
    let content_hash = stable_hash([relative_path.as_bytes(), bytes.as_slice()]);
    let target = item_target(&relative_path);
    let blocked_reason = match target.kind {
        MigrationTargetKind::Unsupported => Some("unsupported file type".to_string()),
        _ => None,
    };
    let status = if blocked_reason.is_some() {
        MigrationItemStatus::Blocked
    } else {
        MigrationItemStatus::Migratable
    };
    let scope = match target.kind {
        MigrationTargetKind::Source => "source-reference",
        MigrationTargetKind::Analysis => "structured-mechanism",
        MigrationTargetKind::Work => "work-outline",
        MigrationTargetKind::Unsupported => "unsupported",
    }
    .to_string();

    Ok(MigrationItem {
        relative_path,
        content_hash,
        target,
        status,
        scope,
        blocked_reason,
    })
}

fn blocked_item(
    root: &Path,
    path: &Path,
    blocked_reason: impl Into<String>,
) -> Result<MigrationItem, MigrationError> {
    let relative_path = relative_path(root, path)?;
    Ok(MigrationItem {
        relative_path,
        content_hash: "fnv1a64:0000000000000000".to_string(),
        target: MigrationItemTarget {
            kind: MigrationTargetKind::Unsupported,
            library_relative_path: "unsupported".to_string(),
        },
        status: MigrationItemStatus::Blocked,
        scope: "unsupported".to_string(),
        blocked_reason: Some(blocked_reason.into()),
    })
}

fn item_target(relative_path: &str) -> MigrationItemTarget {
    let path = Path::new(relative_path);
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let relative_without_extension = path_without_extension(path);

    match extension.as_str() {
        "txt" => MigrationItemTarget {
            kind: MigrationTargetKind::Source,
            library_relative_path: format!("sources/{relative_path}"),
        },
        "md" | "json" => MigrationItemTarget {
            kind: MigrationTargetKind::Analysis,
            library_relative_path: format!("analyses/{relative_without_extension}.json"),
        },
        _ => MigrationItemTarget {
            kind: MigrationTargetKind::Unsupported,
            library_relative_path: "unsupported".to_string(),
        },
    }
}

fn path_without_extension(path: &Path) -> String {
    path.with_extension("")
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str().map(str::to_string),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn source_tree_hash(items: &[MigrationItem]) -> String {
    let mut chunks = Vec::new();
    for item in items {
        chunks.push(item.relative_path.as_bytes());
        chunks.push(item.content_hash.as_bytes());
    }
    stable_hash(chunks)
}

fn deduplicate_source_items(items: &mut Vec<MigrationItem>) {
    let mut seen_paths = HashSet::new();
    items.retain(|item| seen_paths.insert(item.relative_path.clone()));
}

fn mark_target_path_collisions(items: &mut [MigrationItem]) {
    let mut target_counts = HashMap::new();
    for item in items
        .iter()
        .filter(|item| item.status == MigrationItemStatus::Migratable)
    {
        *target_counts
            .entry(item.target.library_relative_path.clone())
            .or_insert(0_usize) += 1;
    }

    for item in items
        .iter_mut()
        .filter(|item| item.status == MigrationItemStatus::Migratable)
    {
        if target_counts
            .get(&item.target.library_relative_path)
            .is_some_and(|count| *count > 1)
        {
            item.status = MigrationItemStatus::Blocked;
            item.scope = "blocked-target-collision".to_string();
            item.blocked_reason = Some(format!(
                "target path collision: {}",
                item.target.library_relative_path
            ));
        }
    }
}

fn stable_hash<'a>(chunks: impl IntoIterator<Item = &'a [u8]>) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for chunk in chunks {
        for byte in chunk {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("fnv1a64:{hash:016x}")
}

fn reject_unsafe_include_path(include_path: &str) -> Result<(), MigrationError> {
    let path = Path::new(include_path);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(MigrationError::UnsafePath(include_path.to_string()));
    }
    Ok(())
}

fn ensure_inside_root(root: &Path, path: &Path, display_path: &str) -> Result<(), MigrationError> {
    if path.starts_with(root) {
        Ok(())
    } else {
        Err(MigrationError::UnsafePath(display_path.to_string()))
    }
}

fn relative_path(root: &Path, path: &Path) -> Result<String, MigrationError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| MigrationError::UnsafePath(path.to_string_lossy().to_string()))?;
    let parts = relative
        .components()
        .map(|component| match component {
            Component::Normal(value) => value
                .to_str()
                .map(str::to_string)
                .ok_or_else(|| MigrationError::NonUtf8Path(path.to_path_buf())),
            _ => Ok(String::new()),
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    Ok(parts.join("/"))
}
