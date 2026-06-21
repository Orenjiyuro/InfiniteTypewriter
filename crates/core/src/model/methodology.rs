use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::IsoDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DraftReviewSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeType {
    Create,
    Update,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub source_id: String,
    pub title: String,
    pub locator: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceLocator {
    pub locator: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCoverage {
    pub scope: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfidenceRating {
    pub level: ConfidenceLevel,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCard {
    pub id: String,
    pub kind: EvidenceCardKind,
    pub source: SourceReference,
    pub evidence: Vec<EvidenceLocator>,
    pub observed_technique: String,
    pub source_function: String,
    pub transferable_mechanism: String,
    pub forbidden_copy: Vec<String>,
    pub coverage: EvidenceCoverage,
    pub confidence: ConfidenceRating,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceCardKind {
    EvidenceCard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReaderContract {
    pub id: String,
    pub kind: ReaderContractKind,
    pub promise: String,
    pub progress: String,
    pub payoff: String,
    pub tone: String,
    pub emotional_target: String,
    pub evidence_card_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReaderContractKind {
    ReaderContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBoundary {
    pub id: String,
    pub kind: KnowledgeBoundaryKind,
    pub pov: String,
    pub narrator_knows: Vec<String>,
    pub reader_knows: Vec<String>,
    pub withheld_information: Vec<String>,
    pub unreliable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KnowledgeBoundaryKind {
    KnowledgeBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingPressure {
    pub id: String,
    pub kind: SettingPressureKind,
    pub setting_element: String,
    pub restriction: String,
    pub resource: String,
    pub cost: String,
    pub choice_impact: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SettingPressureKind {
    SettingPressure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceMechanism {
    pub id: String,
    pub kind: ReferenceMechanismKind,
    pub evidence_card_id: String,
    pub mechanism: String,
    pub transfer_rules: Vec<String>,
    pub forbidden_copies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceMechanismKind {
    ReferenceMechanism,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneCard {
    pub id: String,
    pub kind: SceneCardKind,
    pub work_id: String,
    pub title: String,
    pub pov: String,
    pub knowledge_boundary_id: String,
    pub setting_pressure_ids: Vec<String>,
    pub reader_contract_id: String,
    pub character_goals: Vec<String>,
    pub conflict_agenda: String,
    pub revealed_information: Vec<String>,
    pub hidden_information: Vec<String>,
    pub state_change: String,
    pub turn: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SceneCardKind {
    SceneCard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftReviewItem {
    pub id: String,
    pub kind: DraftReviewItemKind,
    pub target_id: String,
    pub severity: DraftReviewSeverity,
    pub category: String,
    pub finding: String,
    pub recommendation: String,
    pub evidence_card_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DraftReviewItemKind {
    DraftReviewItem,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSetEntry {
    pub target_id: String,
    pub change_type: ChangeType,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSet {
    pub id: String,
    pub kind: ChangeSetKind,
    pub created_at: IsoDateTime,
    pub summary: String,
    pub changes: Vec<ChangeSetEntry>,
    pub affected_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeSetKind {
    ChangeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodologySeedBundle {
    pub id: String,
    pub kind: MethodologySeedBundleKind,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub purpose: String,
    pub evidence_cards: Vec<EvidenceCard>,
    pub reader_contracts: Vec<ReaderContract>,
    pub knowledge_boundaries: Vec<KnowledgeBoundary>,
    pub setting_pressures: Vec<SettingPressure>,
    pub reference_mechanisms: Vec<ReferenceMechanism>,
    pub scene_cards: Vec<SceneCard>,
    pub draft_review_items: Vec<DraftReviewItem>,
    pub change_sets: Vec<ChangeSet>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MethodologySeedBundleKind {
    MethodologySeedBundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCardValidationError {
    missing_fields: Vec<String>,
}

impl EvidenceCardValidationError {
    pub fn missing_fields(&self) -> &[String] {
        &self.missing_fields
    }
}

pub fn validate_evidence_card_value(value: &Value) -> Result<(), EvidenceCardValidationError> {
    let mut missing_fields = Vec::new();

    if value.get("evidence").is_none_or(Value::is_null) {
        missing_fields.push("evidence".to_string());
    } else if value
        .get("evidence")
        .and_then(Value::as_array)
        .is_none_or(Vec::is_empty)
    {
        missing_fields.push("evidence".to_string());
    } else if let Some(evidence_items) = value.get("evidence").and_then(Value::as_array) {
        for (index, evidence) in evidence_items.iter().enumerate() {
            if !has_text(evidence.get("locator")) {
                missing_fields.push(evidence_locator_path(index));
            }
            if !has_text(evidence.get("summary")) {
                missing_fields.push(evidence_summary_path(index));
            }
        }
    }

    if value.get("source").is_none_or(Value::is_null) {
        missing_fields.push("source".to_string());
    } else if let Some(source) = value.get("source") {
        if !has_text(source.get("sourceId")) {
            missing_fields.push("source.sourceId".to_string());
        }
        if !has_text(source.get("title")) {
            missing_fields.push("source.title".to_string());
        }
        if !has_text(source.get("locator")) {
            missing_fields.push("source.locator".to_string());
        }
    }

    if value.get("coverage").is_none_or(Value::is_null) {
        missing_fields.push("coverage".to_string());
    } else if let Some(coverage) = value.get("coverage") {
        if !has_text(coverage.get("scope")) {
            missing_fields.push("coverage.scope".to_string());
        }
        if !has_text(coverage.get("notes")) {
            missing_fields.push("coverage.notes".to_string());
        }
    }

    if value.get("confidence").is_none_or(Value::is_null) {
        missing_fields.push("confidence".to_string());
    } else if let Some(confidence) = value.get("confidence") {
        if confidence.get("level").is_none_or(Value::is_null) {
            missing_fields.push("confidence.level".to_string());
        }
        if !has_text(confidence.get("rationale")) {
            missing_fields.push("confidence.rationale".to_string());
        }
    }

    if missing_fields.is_empty() {
        Ok(())
    } else {
        Err(EvidenceCardValidationError { missing_fields })
    }
}

fn has_text(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.is_empty())
}

fn evidence_locator_path(index: usize) -> String {
    format!("evidence[{index}].locator")
}

fn evidence_summary_path(index: usize) -> String {
    format!("evidence[{index}].summary")
}
