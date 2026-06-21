pub mod context_pack;
pub mod library;
pub mod migration;
pub mod model;

pub use library::{initialize_empty_library, LibraryError};
pub use migration::{
    dry_run_private_corpus_migration, MigrationDryRun, MigrationError, MigrationItem,
    MigrationItemStatus, MigrationItemTarget, MigrationReport, MigrationSource, MigrationTarget,
    MigrationTargetKind,
};
pub use model::breakdown::{
    BookSpecificModule, BookSpecificModuleField, BreakdownConcern, BreakdownProject,
    BreakdownRecipe, BreakdownScope, CompletionGateCheck, CompletionGateLevel, ConcernKind,
    CoverageClaim, EvidenceCard as BreakdownEvidenceCard, FieldEvidenceRef, ModuleDisplayHint,
    ModuleFieldValueKind, ModuleMigrationStrategy, ObservationKind,
    ReferenceMechanism as BreakdownReferenceMechanism, TransferRule, ViewProjection,
    ViewProjectionDisplayKind,
};
pub use model::character::{
    CharacterArc, CharacterDynamicState, CharacterStableProfile, CharacterTimeline,
    FocusRelationshipView, InteractionPattern, RelationshipEdge, RelationshipTemperature,
    RelationshipTemperatureLevel, Score,
};
pub use model::common::{AuditTrailEntry, BaseObject, ContextHints, StableId};
pub use model::context_pack::{
    ActivationReason, Canonicalization, ConfidenceLevel as ContextConfidenceLevel,
    ContextActivation, ContextBudget, ContextExclusion, ContextExclusionReason,
    ContextExclusionRiskKind, ContextHash, ContextIncludedItem, ContextPack, ContextPriority,
    ContextTaskDescriptor, ContextTaskType, EvidenceChainEntry, FragmentBudget, FreshnessState,
    HashAlgorithm, ManifestHashEntry, ObjectBudget, OverflowPolicy, PriorityBands, RelevanceBand,
    SchemaVersionEntry, SourceBreakdownStage, TokenBudget, TrimmingStrategy, UserSelectionState,
};
pub use model::methodology::{
    validate_evidence_card_value, ChangeSet, DraftReviewItem, EvidenceCard, KnowledgeBoundary,
    MethodologySeedBundle, ReaderContract, ReferenceMechanism, SceneCard as MethodologySceneCard,
    SettingPressure,
};
pub use model::simulation::{
    ActionCandidate, OutlineAdjustmentSuggestion, PuppeteeringRisk, SimulationContext,
    SimulationResult,
};
pub use model::story::SceneCard as StorySceneCard;
pub use model::story::{
    Beat, ChangeSet as StoryChangeSet, ChangeSetEntry as StoryChangeSetEntry, ChapterPlan,
    ContinuityIssue, DraftReviewItem as StoryDraftReviewItem, ForeshadowingEntry,
    ForeshadowingLedger, ProseQualityPass, RevisionLayer, RevisionRequest, RevisionSuggestion,
    SceneCard, StorySegment, StyleRevisionPass, Work, WorkOutline,
};
pub use model::style::{BookMechanic, ImageryRecord, MicroTechnique, NarrativeDNA, StyleProfile};
pub use model::{
    create_empty_manifest, AnalysisRecord, BackupRestoreState, BackupRestoreStatus,
    BreakdownProjectRecord, ChangeSetRecord as LibraryChangeSetRecord, CraftReferenceRecord,
    DraftRecord, LibraryExportPolicy, LibraryManifest, LibraryRoot, MigrationStatus,
    ProviderConfigExportMode, ProviderRunRecord, ReferenceBookRecord,
    RevisionRequestRecord as LibraryRevisionRequestRecord, SourceRecord, TransferableAssetRecord,
    WorkRecord,
};
