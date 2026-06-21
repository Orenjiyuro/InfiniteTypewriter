use serde::{Deserialize, Serialize};

pub mod breakdown;
pub mod character;
pub mod common;
pub mod context_pack;
pub mod methodology;
pub mod simulation;
pub mod story;
pub mod style;

pub type IsoDateTime = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryRoot {
    pub id: String,
    pub label: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRecord {
    pub id: String,
    pub kind: SourceRecordKind,
    pub title: String,
    pub content_hash: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceRecordKind {
    Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkRecord {
    pub id: String,
    pub kind: WorkRecordKind,
    pub title: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkRecordKind {
    Work,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisRecord {
    pub id: String,
    pub kind: AnalysisRecordKind,
    pub source_id: String,
    pub title: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisRecordKind {
    Analysis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceBookRecord {
    pub id: String,
    pub kind: ReferenceBookRecordKind,
    pub title: String,
    pub source_id: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceBookRecordKind {
    ReferenceBook,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakdownProjectRecord {
    pub id: String,
    pub kind: BreakdownProjectRecordKind,
    pub title: String,
    pub source_id: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BreakdownProjectRecordKind {
    BreakdownProject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRunRecord {
    pub id: String,
    pub kind: ProviderRunRecordKind,
    pub provider_job_ref: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderRunRecordKind {
    ProviderRun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftRecord {
    pub id: String,
    pub kind: DraftRecordKind,
    pub target_ref: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DraftRecordKind {
    Draft,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionRequestRecord {
    pub id: String,
    pub kind: RevisionRequestRecordKind,
    pub target_ref: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RevisionRequestRecordKind {
    RevisionRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSetRecord {
    pub id: String,
    pub kind: ChangeSetRecordKind,
    pub target_ref: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeSetRecordKind {
    ChangeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableAssetRecord {
    pub id: String,
    pub kind: TransferableAssetRecordKind,
    pub mechanism_ref: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransferableAssetRecordKind {
    TransferableAsset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftReferenceRecord {
    pub id: String,
    pub kind: CraftReferenceRecordKind,
    pub title: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CraftReferenceRecordKind {
    CraftReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackupRestoreState {
    NotRun,
    DryRunReady,
    Confirmed,
    Blocked,
    Applied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRestoreStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_dry_run_at: Option<IsoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_at: Option<IsoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_dry_run_at: Option<IsoDateTime>,
    pub user_confirmation_required: bool,
    pub status: BackupRestoreState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationStatus {
    pub current_schema_version: u16,
    pub pending_migration: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_dry_run_at: Option<IsoDateTime>,
    pub user_confirmation_required: bool,
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderConfigExportMode {
    SummaryOnly,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryExportPolicy {
    pub include_ai_task_audit_records_by_default: bool,
    pub export_provider_secrets: bool,
    pub provider_config_export_mode: ProviderConfigExportMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryManifest {
    pub schema_version: u16,
    pub root: LibraryRoot,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub last_writer_version: String,
    pub sources: Vec<SourceRecord>,
    pub works: Vec<WorkRecord>,
    pub analyses: Vec<AnalysisRecord>,
    pub reference_books: Vec<ReferenceBookRecord>,
    pub breakdown_projects: Vec<BreakdownProjectRecord>,
    pub provider_runs: Vec<ProviderRunRecord>,
    pub drafts: Vec<DraftRecord>,
    pub revision_requests: Vec<RevisionRequestRecord>,
    pub change_sets: Vec<ChangeSetRecord>,
    pub transferable_assets: Vec<TransferableAssetRecord>,
    pub craft_references: Vec<CraftReferenceRecord>,
    pub backup_restore_status: BackupRestoreStatus,
    pub migration_status: MigrationStatus,
    pub export_policy: LibraryExportPolicy,
}

pub fn create_empty_manifest(
    root: LibraryRoot,
    timestamp: impl Into<IsoDateTime>,
) -> LibraryManifest {
    let timestamp = timestamp.into();

    LibraryManifest {
        schema_version: 2,
        root,
        created_at: timestamp.clone(),
        updated_at: timestamp,
        last_writer_version: "unknown-writer".to_string(),
        sources: Vec::new(),
        works: Vec::new(),
        analyses: Vec::new(),
        reference_books: Vec::new(),
        breakdown_projects: Vec::new(),
        provider_runs: Vec::new(),
        drafts: Vec::new(),
        revision_requests: Vec::new(),
        change_sets: Vec::new(),
        transferable_assets: Vec::new(),
        craft_references: Vec::new(),
        backup_restore_status: BackupRestoreStatus {
            last_dry_run_at: None,
            last_backup_at: None,
            last_restore_dry_run_at: None,
            user_confirmation_required: true,
            status: BackupRestoreState::NotRun,
        },
        migration_status: MigrationStatus {
            current_schema_version: 2,
            pending_migration: false,
            last_dry_run_at: None,
            user_confirmation_required: true,
            blocked_reasons: Vec::new(),
        },
        export_policy: LibraryExportPolicy {
            include_ai_task_audit_records_by_default: false,
            export_provider_secrets: false,
            provider_config_export_mode: ProviderConfigExportMode::SummaryOnly,
        },
    }
}
