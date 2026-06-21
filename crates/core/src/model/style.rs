use serde::{Deserialize, Serialize};

use super::common::{BaseObject, StableId};
use super::context_pack::ContextTaskType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleProfile {
    #[serde(flatten)]
    pub base: BaseObject,
    pub owner_ref: StableId,
    pub scope: String,
    pub style_dimensions: Vec<String>,
    pub voice_constraints: Vec<String>,
    pub do_not_imitate: Vec<String>,
    pub evidence_ref_ids: Vec<StableId>,
    pub transfer_rule_ids: Vec<StableId>,
    pub misuse_risks: Vec<String>,
    pub task_use_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NarrativeDNA {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub reader_experience_goal: String,
    pub promise_progress_payoff_pattern: String,
    pub tension_engine: String,
    pub scene_rhythm: String,
    pub signature_moves: Vec<String>,
    pub evidence_ref_ids: Vec<StableId>,
    pub coverage: String,
    pub confidence: String,
    pub mechanism_ref_ids: Vec<StableId>,
    pub transfer_rule_refs: Vec<StableId>,
    pub applicable_task_types: Vec<ContextTaskType>,
    pub applicable_scope: String,
    pub misuse_risks: Vec<String>,
    pub do_not_copy: Vec<String>,
    pub prohibited_residue: Vec<String>,
    pub current_work_conflict_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroTechnique {
    #[serde(flatten)]
    pub base: BaseObject,
    pub name: String,
    pub mechanism: String,
    pub use_case: String,
    pub constraints: Vec<String>,
    pub source_evidence_refs: Vec<StableId>,
    pub transfer_rule_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageryRecord {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub image_label: String,
    pub meaning: String,
    pub first_use_ref: StableId,
    pub recurrence_rules: Vec<String>,
    pub current_state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookMechanic {
    #[serde(flatten)]
    pub base: BaseObject,
    pub owner_ref: StableId,
    pub mechanic_name: String,
    pub mechanic_purpose: String,
    pub operating_rules: Vec<String>,
    pub failure_modes: Vec<String>,
    pub evidence_ref_ids: Vec<StableId>,
}
