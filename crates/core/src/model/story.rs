use serde::{Deserialize, Serialize};

use super::common::{BaseObject, IsoDateTime, StableId};
use super::context_pack::ContextTaskType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_kind: String,
    pub premise: String,
    pub reader_contract_refs: Vec<StableId>,
    pub outline: WorkOutline,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkOutline {
    pub id: StableId,
    pub work_id: StableId,
    pub volumes: Vec<VolumeOutline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeOutline {
    pub id: StableId,
    pub work_id: StableId,
    pub title: String,
    pub sequence: u32,
    pub purpose: String,
    pub segments: Vec<StorySegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorySegment {
    pub id: StableId,
    pub work_id: StableId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<StableId>,
    pub label: String,
    pub sequence: u32,
    pub goal: String,
    pub pressure: String,
    pub chapters: Vec<ChapterPlan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterPlan {
    pub id: StableId,
    pub work_id: StableId,
    pub sequence: u32,
    pub title: String,
    pub chapter_goal: String,
    pub scene_refs: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beat {
    pub id: StableId,
    pub scene_id: StableId,
    pub sequence: u32,
    pub beat_kind: String,
    pub intent: String,
    pub expected_change: String,
    pub character_refs: Vec<StableId>,
    pub evidence_ref_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneCard {
    #[serde(flatten)]
    pub base: BaseObject,
    pub chapter_id: StableId,
    pub scene_goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pov_character_id: Option<StableId>,
    pub pov_mode: String,
    pub knowledge_boundary_ref: StableId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_pressure_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_pressure_text: Option<String>,
    pub entry_state: String,
    pub exit_state: String,
    pub character_state_refs: Vec<StableId>,
    pub relationship_refs: Vec<StableId>,
    pub foreshadowing_refs: Vec<StableId>,
    pub style_target_refs: Vec<StableId>,
    pub beats: Vec<Beat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeshadowingEntry {
    pub id: StableId,
    pub ledger_id: StableId,
    pub signal: String,
    pub planted_at_ref: StableId,
    pub intended_payoff: String,
    pub current_state: String,
    pub visibility: String,
    pub risk_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeshadowingLedger {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub entries: Vec<ForeshadowingEntry>,
    pub last_reviewed_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionLayer {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub layer_kind: String,
    pub objective: String,
    pub review_item_refs: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuityIssue {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub issue_kind: String,
    pub description: String,
    pub related_refs: Vec<StableId>,
    pub severity: String,
    pub proposed_fix: String,
    pub confirmation_state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftReviewItem {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub source_draft_ref: StableId,
    pub severity: String,
    pub category: String,
    pub finding: String,
    pub recommendation: String,
    pub evidence_ref_ids: Vec<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_refs: Option<Vec<StableId>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionRequest {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub original_chunk_ref: StableId,
    pub original_draft_ref: StableId,
    pub user_feedback: String,
    pub output_schema_ref: String,
    pub output_schema_version: u16,
    pub context_hash: String,
    pub requested_rerun_task_type: ContextTaskType,
    pub request_status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSetEntry {
    pub target_ref: StableId,
    pub change_type: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSet {
    #[serde(flatten)]
    pub base: BaseObject,
    pub summary: String,
    pub reason: String,
    pub source_draft_ref: StableId,
    pub revision_request_refs: Vec<StableId>,
    pub target_refs: Vec<StableId>,
    pub changes: Vec<ChangeSetEntry>,
    pub impact_scope: Vec<String>,
    pub confirmation_state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionSuggestion {
    pub target_ref: StableId,
    pub suggested_text_ref: StableId,
    pub reason: String,
    pub evidence_ref_ids: Vec<StableId>,
    pub risk_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProseQualityPass {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub quality_goals: Vec<String>,
    pub before_ref: StableId,
    pub suggestions: Vec<RevisionSuggestion>,
    pub decision: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleRevisionPass {
    #[serde(flatten)]
    pub base: BaseObject,
    pub target_ref: StableId,
    pub style_profile_refs: Vec<StableId>,
    pub voice_constraints: Vec<String>,
    pub suggestions: Vec<RevisionSuggestion>,
    pub decision: String,
}
