use serde::{Deserialize, Serialize};

use super::common::{BaseObject, StableId};
use super::context_pack::ContextTaskType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConcernKind {
    Outline,
    Character,
    Scene,
    Setting,
    Worldbuilding,
    Pov,
    ReaderContract,
    Style,
    Craft,
    Relationship,
    Foreshadowing,
    Revision,
    Custom,
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
pub enum ModuleFieldValueKind {
    Text,
    Number,
    Boolean,
    StringList,
    RecordList,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleDisplayHint {
    Card,
    Table,
    Timeline,
    Graph,
    Hidden,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ViewProjectionDisplayKind {
    Card,
    Table,
    Timeline,
    Graph,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleMigrationStrategy {
    Preserve,
    DropIfUnknown,
    ManualReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompletionGateLevel {
    Setup,
    Stage,
    Volume,
    Book,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObservationKind {
    Observation,
    Inference,
    Assumption,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageClaim {
    pub scope: String,
    pub unavailable_scope: Vec<String>,
    pub evidence_ref_ids: Vec<StableId>,
    pub confidence: ConfidenceLevel,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldEvidenceRef {
    pub field_path: String,
    pub evidence_ref_ids: Vec<StableId>,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRule {
    pub id: StableId,
    pub source_fact: String,
    pub transferable_mechanism: String,
    pub target_variable_slots: Vec<String>,
    pub prohibited_residue: Vec<String>,
    pub misuse_risks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCard {
    #[serde(flatten)]
    pub base: BaseObject,
    pub breakdown_project_id: StableId,
    pub breakdown_scope_id: StableId,
    pub source_locator: String,
    pub observation_kind: ObservationKind,
    pub observed_technique: String,
    pub function_in_source: String,
    pub transferable_mechanism: String,
    pub prohibited_copying: Vec<String>,
    pub coverage: CoverageClaim,
    pub confidence: ConfidenceLevel,
    pub field_evidence_refs: Vec<FieldEvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceMechanism {
    #[serde(flatten)]
    pub base: BaseObject,
    pub source_evidence_refs: Vec<StableId>,
    pub mechanism: String,
    pub transfer_rule_ids: Vec<StableId>,
    pub variable_slots: Vec<String>,
    pub prohibited_residue: Vec<String>,
    pub misuse_risks: Vec<String>,
    pub applicable_task_types: Vec<ContextTaskType>,
    pub applicable_scope: String,
    pub current_work_conflict_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakdownConcern {
    pub id: StableId,
    pub kind: ConcernKind,
    pub label: String,
    pub required_for_recipe: bool,
    pub coverage: CoverageClaim,
    pub field_evidence_refs: Vec<FieldEvidenceRef>,
    pub transfer_rule_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSpecificModuleField {
    pub name: String,
    pub value_kind: ModuleFieldValueKind,
    pub evidence_required: bool,
    pub display_hint: ModuleDisplayHint,
    pub migration_strategy: ModuleMigrationStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSpecificModule {
    pub id: StableId,
    pub label: String,
    pub applies_to_concern_ids: Vec<StableId>,
    pub fields: Vec<BookSpecificModuleField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewProjection {
    pub id: StableId,
    pub label: String,
    pub target_concern_id: StableId,
    pub source_field_paths: Vec<String>,
    pub display_kind: ViewProjectionDisplayKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionGateCheck {
    pub id: StableId,
    pub target_ref: StableId,
    pub gate_level: CompletionGateLevel,
    pub check_kind: String,
    pub blocking: bool,
    pub passed: bool,
    pub evidence_ref_ids: Vec<StableId>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakdownRecipe {
    #[serde(flatten)]
    pub base: BaseObject,
    pub concerns: Vec<BreakdownConcern>,
    pub transfer_rules: Vec<TransferRule>,
    pub modules: Vec<BookSpecificModule>,
    pub view_projections: Vec<ViewProjection>,
    pub completion_gate_checks: Vec<CompletionGateCheck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakdownProject {
    #[serde(flatten)]
    pub base: BaseObject,
    pub source_id: StableId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_method: Option<String>,
    pub objective: String,
    pub scope_ids: Vec<StableId>,
    pub output_ref_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakdownScope {
    #[serde(flatten)]
    pub base: BaseObject,
    pub breakdown_project_id: StableId,
    pub scope_label: String,
    pub locator: String,
    pub coverage_intent: String,
    pub exclusions: Vec<String>,
}
