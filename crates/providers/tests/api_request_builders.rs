use infinite_typewriter_providers::api::{
    anthropic::build_anthropic_request, gemini::build_gemini_request,
    ollama::build_ollama_generate_request, openai_compatible::build_openai_responses_request,
    ApiRequestBuildInput,
};
use infinite_typewriter_providers::secret::ResolvedSecret;
use infinite_typewriter_providers::ProviderKind;

const TOY_ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn openai_compatible_builder_redacts_bearer_auth_and_records_contract_metadata() {
    let request = build_openai_responses_request(
        input("api-openai-compatible"),
        &ResolvedSecret::new("sk-test-secret-value"),
    );

    assert_eq!(
        request.summary.provider_kind,
        ProviderKind::ApiOpenaiCompatible
    );
    assert_eq!(request.summary.endpoint_family, "responses");
    assert_eq!(
        request.summary.auth_headers["authorization"],
        "[redacted:auth]"
    );
    assert_eq!(
        request.summary.prompt_template_version,
        "draft-generation-v1"
    );
    assert_eq!(
        request.summary.output_schema_ref.as_deref(),
        Some("toy-draft-output")
    );
    assert_eq!(request.body["model"], "toy-model");
}

#[test]
fn anthropic_builder_separates_system_and_user_context_and_records_usage_fields() {
    let request = build_anthropic_request(
        input("api-anthropic"),
        &ResolvedSecret::new("anthropic-secret"),
    );

    assert_eq!(request.summary.provider_kind, ProviderKind::ApiAnthropic);
    assert_eq!(request.summary.endpoint_family, "messages");
    assert_eq!(request.summary.auth_headers["x-api-key"], "[redacted:auth]");
    assert_eq!(request.body["system"], "system instruction");
    assert_eq!(
        request.summary.expected_usage_fields,
        vec!["inputTokens", "outputTokens"]
    );
    assert!(request
        .summary
        .output_summary_fields
        .contains(&"stopReason".to_string()));
}

#[test]
fn gemini_builder_records_structured_output_and_usage_metadata() {
    let request = build_gemini_request(input("api-gemini"), &ResolvedSecret::new("gemini-secret"));

    assert_eq!(request.summary.provider_kind, ProviderKind::ApiGemini);
    assert_eq!(request.summary.endpoint_family, "generateContent");
    assert_eq!(
        request.summary.auth_headers["x-goog-api-key"],
        "[redacted:auth]"
    );
    assert_eq!(
        request.body["system_instruction"]["parts"][0]["text"],
        "system instruction"
    );
    assert_eq!(
        request.body["generationConfig"]["responseMimeType"],
        "application/json"
    );
    assert!(request
        .summary
        .output_summary_fields
        .contains(&"finishReason".to_string()));
    assert!(request
        .summary
        .expected_usage_fields
        .contains(&"usageMetadata".to_string()));
}

#[test]
fn ollama_builder_uses_local_service_without_remote_secret() {
    let request = build_ollama_generate_request(input("api-ollama"));

    assert_eq!(request.summary.provider_kind, ProviderKind::ApiOllama);
    assert_eq!(request.summary.endpoint_family, "/api/generate");
    assert!(request.summary.auth_headers.is_empty());
    assert_eq!(request.body["stream"], false);
    assert_eq!(
        request.summary.expected_usage_fields,
        vec!["promptEvalCount", "evalCount", "doneReason"]
    );
}

fn input(provider_kind: &str) -> ApiRequestBuildInput {
    ApiRequestBuildInput {
        provider_kind: provider_kind.to_string(),
        base_url: "https://provider.example/v1".to_string(),
        model: "toy-model".to_string(),
        system_instruction: "system instruction".to_string(),
        user_context: "toy public context".to_string(),
        output_schema_ref: Some("toy-draft-output".to_string()),
        prompt_template_version: "draft-generation-v1".to_string(),
        context_hash_value: TOY_ZERO_HASH.to_string(),
        timeout_ms: 30000,
    }
}
