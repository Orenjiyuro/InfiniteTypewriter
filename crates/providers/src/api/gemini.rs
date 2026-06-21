use std::collections::BTreeMap;

use serde_json::json;

use crate::api::{ApiRequestBuildInput, ApiRequestSummary, BuiltApiRequest};
use crate::model::ProviderKind;
use crate::redaction::redact_headers;
use crate::secret::ResolvedSecret;

pub fn build_gemini_request(
    input: ApiRequestBuildInput,
    secret: &ResolvedSecret,
) -> BuiltApiRequest {
    let mut headers = BTreeMap::new();
    headers.insert(
        "x-goog-api-key".to_string(),
        secret.expose_for_request_construction().to_string(),
    );
    let auth_headers = redact_headers(&headers).value;

    BuiltApiRequest {
        method: "POST".to_string(),
        url: format!(
            "{}/models/{}:generateContent",
            input.base_url.trim_end_matches('/'),
            input.model
        ),
        headers,
        body: json!({
            "system_instruction": {
                "parts": [{ "text": input.system_instruction }]
            },
            "contents": [{
                "role": "user",
                "parts": [{ "text": input.user_context }]
            }],
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseSchemaRef": input.output_schema_ref
            },
            "metadata": {
                "contextHash": input.context_hash_value,
                "promptTemplateVersion": input.prompt_template_version
            }
        }),
        timeout_ms: input.timeout_ms,
        summary: ApiRequestSummary {
            provider_kind: ProviderKind::ApiGemini,
            endpoint_family: "generateContent".to_string(),
            auth_headers,
            prompt_template_version: input.prompt_template_version,
            output_schema_ref: input.output_schema_ref,
            context_hash_value: input.context_hash_value,
            expected_usage_fields: vec!["usageMetadata".to_string()],
            output_summary_fields: vec!["finishReason".to_string(), "safetyBlock".to_string()],
        },
    }
}
