use serde::{Deserialize, Serialize};

use super::common::{BaseObject, IsoDateTime, StableId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextTaskType {
    SourceBreakdown,
    CreativeDiscussion,
    OutlineGeneration,
    CharacterSimulation,
    ScenePlanning,
    DraftGeneration,
    StyleRevision,
    ContinuityAudit,
    WritebackSuggestion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceBreakdownStage {
    SegmentExtraction,
    ParentSynthesis,
    CrossDimensionReview,
    DraftReviewRerun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextTaskDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<ContextTaskType>,
    pub source_breakdown_stage: SourceBreakdownStage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_refs: Option<Vec<StableId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_refs: Option<Vec<StableId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_chunk_refs: Option<Vec<StableId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_chunk_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_draft_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_request_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema_version: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivationReason {
    TargetMatch,
    DirectReference,
    ContextHintKeyword,
    TaskDefault,
    ChangeSetImpact,
    ContinuityDependency,
    UserSelected,
    PinnedRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceChainEntry {
    pub r#ref: StableId,
    pub relation: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelevanceBand {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextPriority {
    Required,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FreshnessState {
    Current,
    Stale,
    Superseded,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserSelectionState {
    Auto,
    UserIncluded,
    UserExcluded,
    UserPinned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextActivation {
    pub id: StableId,
    pub context_pack_id: StableId,
    pub source_ref: StableId,
    pub source_object_type: String,
    pub source_schema_version: u16,
    pub activation_reason: ActivationReason,
    pub evidence_chain: Vec<EvidenceChainEntry>,
    pub relevance: RelevanceBand,
    pub confidence: ConfidenceLevel,
    pub priority: ContextPriority,
    pub freshness_state: FreshnessState,
    pub user_selection: UserSelectionState,
    pub selected_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBudget {
    pub max_input_tokens: u32,
    pub reserved_output_tokens: u32,
    pub estimation_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FragmentBudget {
    pub max_fragments: u32,
    pub max_fragment_chars: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectBudget {
    pub object_type: String,
    pub max_objects: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityBands {
    pub required: u32,
    pub high: u32,
    pub normal: u32,
    pub low: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrimmingStrategy {
    DropLowPriority,
    SummarizeEligible,
    LatestConfirmedFirst,
    ClosestScopeFirst,
    UserPinnedFirst,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OverflowPolicy {
    FailClosed,
    DropWithExclusionRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextBudget {
    pub id: StableId,
    pub context_pack_id: StableId,
    pub token_budget: TokenBudget,
    pub fragment_budget: FragmentBudget,
    pub object_budget: Vec<ObjectBudget>,
    pub priority_bands: PriorityBands,
    pub trimming_strategy: TrimmingStrategy,
    pub excluded_object_types: Vec<String>,
    pub excluded_refs: Vec<StableId>,
    pub overflow_policy: OverflowPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestHashEntry {
    pub r#ref: StableId,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaVersionEntry {
    pub schema_id: String,
    pub version: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Canonicalization {
    JsonCanonicalV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextHash {
    pub algorithm: HashAlgorithm,
    pub canonicalization: Canonicalization,
    pub schema_version_map: Vec<SchemaVersionEntry>,
    pub library_manifest_hash: String,
    pub source_hash_refs: Vec<ManifestHashEntry>,
    pub change_set_refs: Vec<StableId>,
    pub change_set_hash: String,
    pub selection_policy_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template_version: Option<String>,
    pub included_item_hashes: Vec<ManifestHashEntry>,
    pub activation_hashes: Vec<ManifestHashEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_item_hashes: Option<Vec<ManifestHashEntry>>,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextExclusionReason {
    BudgetInsufficient,
    StaleOrSuperseded,
    UserExcluded,
    PermissionBoundary,
    TaskIrrelevant,
    InsufficientEvidence,
    DuplicateOrCovered,
    SchemaUnsupported,
    SourceBoundary,
    NotInSelectedScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextExclusionRiskKind {
    PrivacyBoundary,
    CopyrightBoundary,
    StaleState,
    BudgetOverflow,
    Irrelevant,
    LowConfidence,
    UserChoice,
    PermissionDenied,
    UnsupportedSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextExclusion {
    pub id: StableId,
    pub context_pack_id: StableId,
    pub excluded_ref: StableId,
    pub source_object_type: String,
    pub reason: ContextExclusionReason,
    pub risk_kind: ContextExclusionRiskKind,
    pub explanation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_score: Option<RelevanceBand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub would_fit_if_budget: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_activation_ref: Option<StableId>,
    pub created_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextIncludedItem {
    pub source_ref: StableId,
    pub source_object_type: String,
    pub source_schema_version: u16,
    pub content_hash: String,
    pub activation_ref: StableId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextPack {
    #[serde(flatten)]
    pub base: BaseObject,
    pub task_type: ContextTaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_descriptor: Option<ContextTaskDescriptor>,
    pub target_ref: StableId,
    pub activations: Vec<ContextActivation>,
    pub included_items: Vec<ContextIncludedItem>,
    pub exclusions: Vec<ContextExclusion>,
    pub budget: ContextBudget,
    pub context_hash: ContextHash,
    pub created_for_task_ref: StableId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_for_job_ref: Option<StableId>,
}
