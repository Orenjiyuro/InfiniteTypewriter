use std::collections::BTreeMap;

use serde_json::Value;

use crate::model::ProviderKind;

pub mod anthropic;
pub mod gemini;
pub mod ollama;
pub mod openai_compatible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRequestBuildInput {
    pub provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub system_instruction: String,
    pub user_context: String,
    pub output_schema_ref: Option<String>,
    pub prompt_template_version: String,
    pub context_hash_value: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltApiRequest {
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub body: Value,
    pub timeout_ms: u64,
    pub summary: ApiRequestSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRequestSummary {
    pub provider_kind: ProviderKind,
    pub endpoint_family: String,
    pub auth_headers: BTreeMap<String, String>,
    pub prompt_template_version: String,
    pub output_schema_ref: Option<String>,
    pub context_hash_value: String,
    pub expected_usage_fields: Vec<String>,
    pub output_summary_fields: Vec<String>,
}
