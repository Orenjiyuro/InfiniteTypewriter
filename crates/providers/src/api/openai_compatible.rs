use std::collections::BTreeMap;

use serde_json::json;

use crate::api::{ApiRequestBuildInput, ApiRequestSummary, BuiltApiRequest};
use crate::model::ProviderKind;
use crate::redaction::redact_headers;
use crate::secret::ResolvedSecret;

pub fn build_openai_responses_request(
    input: ApiRequestBuildInput,
    secret: &ResolvedSecret,
) -> BuiltApiRequest {
    let mut headers = BTreeMap::new();
    headers.insert(
        "authorization".to_string(),
        format!("Bearer {}", secret.expose_for_request_construction()),
    );
    headers.insert("content-type".to_string(), "application/json".to_string());
    let auth_headers = redact_headers(&headers).value;

    BuiltApiRequest {
        method: "POST".to_string(),
        url: format!("{}/responses", input.base_url.trim_end_matches('/')),
        headers,
        body: json!({
            "model": input.model,
            "input": [
                { "role": "system", "content": input.system_instruction },
                { "role": "user", "content": input.user_context }
            ],
            "text": {
                "format": {
                    "type": "json_schema",
                    "name": input.output_schema_ref.clone().unwrap_or_else(|| "unstructured-output".to_string())
                }
            },
            "metadata": {
                "contextHash": input.context_hash_value,
                "promptTemplateVersion": input.prompt_template_version
            }
        }),
        timeout_ms: input.timeout_ms,
        summary: ApiRequestSummary {
            provider_kind: ProviderKind::ApiOpenaiCompatible,
            endpoint_family: "responses".to_string(),
            auth_headers,
            prompt_template_version: input.prompt_template_version,
            output_schema_ref: input.output_schema_ref,
            context_hash_value: input.context_hash_value,
            expected_usage_fields: vec![
                "inputTokens".to_string(),
                "outputTokens".to_string(),
                "totalTokens".to_string(),
            ],
            output_summary_fields: vec!["responseStatus".to_string(), "requestId".to_string()],
        },
    }
}
