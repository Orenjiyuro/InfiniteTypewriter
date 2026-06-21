use serde::{Deserialize, Serialize};

use super::common::{BaseObject, StableId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionCandidate {
    pub id: StableId,
    pub simulation_context_id: StableId,
    pub actor_id: StableId,
    pub action: String,
    pub motivation: String,
    pub expected_consequence: String,
    pub supporting_refs: Vec<StableId>,
    pub risk_flags: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuppeteeringRisk {
    pub id: StableId,
    pub action_candidate_id: StableId,
    pub risk_kind: String,
    pub conflicting_refs: Vec<StableId>,
    pub severity: String,
    pub explanation: String,
    pub repair_options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationContext {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub task_ref: StableId,
    pub scene_or_segment_ref: StableId,
    pub character_refs: Vec<StableId>,
    pub stable_profile_refs: Vec<StableId>,
    pub dynamic_state_refs: Vec<StableId>,
    pub relationship_refs: Vec<StableId>,
    pub constraints: Vec<String>,
    pub question: String,
    pub candidate_actions: Vec<ActionCandidate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationResult {
    #[serde(flatten)]
    pub base: BaseObject,
    pub simulation_context_id: StableId,
    pub candidate_refs: Vec<StableId>,
    pub recommended_direction: String,
    pub required_user_decisions: Vec<String>,
    pub draft_change_set_refs: Vec<StableId>,
    pub outline_adjustment_suggestion_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutlineAdjustmentSuggestion {
    pub id: StableId,
    pub simulation_result_id: StableId,
    pub target_outline_ref: StableId,
    pub current_constraint: String,
    pub suggested_change: String,
    pub rationale: String,
    pub risk_if_not_adjusted: String,
}
