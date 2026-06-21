use std::collections::BTreeMap;

use serde_json::json;

use crate::api::{ApiRequestBuildInput, ApiRequestSummary, BuiltApiRequest};
use crate::model::ProviderKind;
use crate::redaction::redact_headers;
use crate::secret::ResolvedSecret;

pub fn build_anthropic_request(
    input: ApiRequestBuildInput,
    secret: &ResolvedSecret,
) -> BuiltApiRequest {
    let mut headers = BTreeMap::new();
    headers.insert(
        "x-api-key".to_string(),
        secret.expose_for_request_construction().to_string(),
    );
    headers.insert(
        "anthropic-version".to_string(),
        "configured-version".to_string(),
    );
    let auth_headers = redact_headers(&headers).value;

    BuiltApiRequest {
        method: "POST".to_string(),
        url: format!("{}/messages", input.base_url.trim_end_matches('/')),
        headers,
        body: json!({
            "model": input.model,
            "system": input.system_instruction,
            "messages": [
                { "role": "user", "content": input.user_context }
            ],
            "metadata": {
                "contextHash": input.context_hash_value,
                "promptTemplateVersion": input.prompt_template_version,
                "outputSchemaRef": input.output_schema_ref
            }
        }),
        timeout_ms: input.timeout_ms,
        summary: ApiRequestSummary {
            provider_kind: ProviderKind::ApiAnthropic,
            endpoint_family: "messages".to_string(),
            auth_headers,
            prompt_template_version: input.prompt_template_version,
            output_schema_ref: input.output_schema_ref,
            context_hash_value: input.context_hash_value,
            expected_usage_fields: vec!["inputTokens".to_string(), "outputTokens".to_string()],
            output_summary_fields: vec!["stopReason".to_string(), "requestId".to_string()],
        },
    }
}
