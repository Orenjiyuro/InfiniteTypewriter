use std::collections::BTreeMap;

use serde_json::json;

use crate::api::{ApiRequestBuildInput, ApiRequestSummary, BuiltApiRequest};
use crate::model::ProviderKind;

pub fn build_ollama_generate_request(input: ApiRequestBuildInput) -> BuiltApiRequest {
    BuiltApiRequest {
        method: "POST".to_string(),
        url: format!("{}/api/generate", input.base_url.trim_end_matches('/')),
        headers: BTreeMap::new(),
        body: json!({
            "model": input.model,
            "system": input.system_instruction,
            "prompt": input.user_context,
            "stream": false,
            "format": if input.output_schema_ref.is_some() { "json" } else { "" },
            "options": {
                "contextHash": input.context_hash_value,
                "promptTemplateVersion": input.prompt_template_version
            }
        }),
        timeout_ms: input.timeout_ms,
        summary: ApiRequestSummary {
            provider_kind: ProviderKind::ApiOllama,
            endpoint_family: "/api/generate".to_string(),
            auth_headers: BTreeMap::new(),
            prompt_template_version: input.prompt_template_version,
            output_schema_ref: input.output_schema_ref,
            context_hash_value: input.context_hash_value,
            expected_usage_fields: vec![
                "promptEvalCount".to_string(),
                "evalCount".to_string(),
                "doneReason".to_string(),
            ],
            output_summary_fields: vec!["doneReason".to_string()],
        },
    }
}
