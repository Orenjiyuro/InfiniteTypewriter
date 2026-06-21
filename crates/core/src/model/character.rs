use serde::{de::Error as DeserializeError, Deserialize, Deserializer, Serialize};
use serde_json::Number;

use super::common::{BaseObject, IsoDateTime, StableId};

pub type Score = Number;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStableProfile {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub display_name: String,
    pub role: String,
    pub core_drive: String,
    pub core_drive_evidence_ref_ids: Vec<StableId>,
    pub fear_or_wound: String,
    pub boundaries: Vec<String>,
    pub boundary_evidence_ref_ids: Vec<StableId>,
    pub competencies: Vec<String>,
    pub values: Vec<String>,
    pub voice_notes: Vec<String>,
    pub voice_evidence_ref_ids: Vec<StableId>,
    pub action_fingerprints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterDynamicState {
    #[serde(flatten)]
    pub base: BaseObject,
    pub character_id: StableId,
    pub scope_ref: StableId,
    pub timepoint: String,
    pub emotional_state: String,
    pub knowledge_state: Vec<String>,
    pub resources: Vec<String>,
    pub constraints: Vec<String>,
    pub evidence_ref_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterArc {
    #[serde(flatten)]
    pub base: BaseObject,
    pub character_id: StableId,
    pub arc_goal: String,
    pub arc_goal_evidence_ref_ids: Vec<StableId>,
    pub starting_belief: String,
    pub pressure_pattern: String,
    pub turning_points: Vec<StableId>,
    pub current_stage: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterTimelineEntry {
    pub timepoint: String,
    pub event_ref: StableId,
    pub state_delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_ref: Option<StableId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterTimeline {
    #[serde(flatten)]
    pub base: BaseObject,
    pub character_id: StableId,
    pub entries: Vec<CharacterTimelineEntry>,
    pub last_verified_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipEdge {
    #[serde(flatten)]
    pub base: BaseObject,
    pub work_id: StableId,
    pub participant_ids: Vec<StableId>,
    pub relationship_kind: String,
    pub current_temperature: RelationshipTemperatureLevel,
    #[serde(deserialize_with = "deserialize_score")]
    pub trust: Score,
    #[serde(deserialize_with = "deserialize_score")]
    pub tension: Score,
    pub public_status: String,
    pub private_status: String,
    pub evidence_ref_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipTemperatureLevel {
    Cold,
    Cool,
    Neutral,
    Warm,
    Hot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipTemperature {
    #[serde(flatten)]
    pub base: BaseObject,
    pub relationship_edge_id: StableId,
    pub scope_ref: StableId,
    #[serde(deserialize_with = "deserialize_score")]
    pub affinity: Score,
    #[serde(deserialize_with = "deserialize_score")]
    pub conflict: Score,
    #[serde(deserialize_with = "deserialize_score")]
    pub dependency: Score,
    pub recent_triggers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusRelationshipView {
    #[serde(flatten)]
    pub base: BaseObject,
    pub relationship_edge_id: StableId,
    pub task_type: String,
    pub included_state_refs: Vec<StableId>,
    pub excluded_state_refs: Vec<StableId>,
    pub activation_reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionPattern {
    #[serde(flatten)]
    pub base: BaseObject,
    pub participant_role_pattern: String,
    pub trigger: String,
    pub typical_response: String,
    pub variation_rules: Vec<String>,
    pub risk_notes: Vec<String>,
}

fn deserialize_score<'de, D>(deserializer: D) -> Result<Score, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Number::deserialize(deserializer)?;
    let score = value
        .as_f64()
        .ok_or_else(|| D::Error::custom("score must be a finite JSON number"))?;
    if (0.0..=10.0).contains(&score) {
        Ok(value)
    } else {
        Err(D::Error::custom("score must be between 0 and 10"))
    }
}
