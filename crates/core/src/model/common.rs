use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type IsoDateTime = String;
pub type StableId = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetadataValue {
    Text(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObjectStatus {
    Draft,
    Active,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuditTrailAction {
    Create,
    Update,
    Confirm,
    Reject,
    Archive,
    Migrate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuditTrailActor {
    User,
    System,
    Provider,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditTrailEntry {
    pub at: IsoDateTime,
    pub action: AuditTrailAction,
    pub actor: AuditTrailActor,
    pub reason: String,
    pub source_ref_ids: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextBudgetHint {
    Minimal,
    Standard,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextHints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<ContextPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_hint: Option<ContextBudgetHint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseObject {
    pub schema_version: u16,
    pub id: StableId,
    #[serde(rename = "type")]
    pub object_type: String,
    pub label: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub status: ObjectStatus,
    pub tags: Vec<String>,
    pub metadata: BTreeMap<String, MetadataValue>,
    pub audit_trail: Vec<AuditTrailEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_hints: Option<ContextHints>,
}
