use infinite_typewriter_providers::runner::{
    successful_draft_result, validate_runner_preflight, RunnerPreflightError, RunnerPreflightInput,
};
use infinite_typewriter_providers::{ProviderAuthMode, ProviderConfig, ProviderJob};
use serde_json::json;

const TOY_ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn runner_refuses_fail_closed_preflight_gaps() {
    let job = fixture_job();
    let config = fixture_config(true, ProviderAuthMode::None, false);

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            job: &job,
            provider_config: None,
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::MissingProviderConfig)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&fixture_config(false, ProviderAuthMode::None, false)),
            job: &job,
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::ProviderDisabled)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&fixture_config(true, ProviderAuthMode::EnvVar, false)),
            job: &job,
            secret_resolved: false,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::SecretUnresolved)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&config),
            job: &job_without_context_hash(),
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::MissingContextHash)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&config),
            job: &job_without_prompt_template(),
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::MissingPromptTemplateVersion)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&config),
            job: &job,
            secret_resolved: true,
            output_schema_available: false,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::MissingOutputSchema)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&config),
            job: &job,
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: true,
            advanced_agent_mode_requested: false,
        }),
        Err(RunnerPreflightError::RequiredContextBudgetBlocked)
    );

    assert_eq!(
        validate_runner_preflight(RunnerPreflightInput {
            provider_config: Some(&config),
            job: &job,
            secret_resolved: true,
            output_schema_available: true,
            required_context_budget_blocked: false,
            advanced_agent_mode_requested: true,
        }),
        Err(RunnerPreflightError::AdvancedAgentModeDisabled)
    );
}

#[test]
fn runner_accepts_ready_job_and_success_result_is_draft_only() {
    let job = fixture_job();
    let config = fixture_config(true, ProviderAuthMode::None, false);

    validate_runner_preflight(RunnerPreflightInput {
        job: &job,
        provider_config: Some(&config),
        secret_resolved: true,
        output_schema_available: true,
        required_context_budget_blocked: false,
        advanced_agent_mode_requested: false,
    })
    .expect("ready job passes preflight");

    let result = successful_draft_result(&job, "draft-output-fake-toy");
    assert_eq!(result.draft_output_ref, "draft-output-fake-toy");
    assert!(!result.long_term_write_event_emitted);
}

fn job_without_context_hash() -> ProviderJob {
    let mut job = fixture_job();
    job.context_hash.value.clear();
    job
}

fn job_without_prompt_template() -> ProviderJob {
    let mut job = fixture_job();
    job.prompt_template_version.clear();
    job
}

fn fixture_job() -> ProviderJob {
    serde_json::from_value(json!({
        "schemaVersion": 1,
        "id": "provider-job-fake-toy",
        "type": "provider-job",
        "label": "Toy Fake Provider Job",
        "createdAt": "2026-06-06T00:00:00.000Z",
        "updatedAt": "2026-06-06T00:00:00.000Z",
        "status": "ready-for-provider",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "taskType": "draft-generation",
        "targetRef": "scene-card-toy",
        "contextPackRef": "context-pack-draft-toy",
        "contextHash": {
            "algorithm": "sha256",
            "canonicalization": "json-canonical-v1",
            "schemaVersionMap": [{ "schemaId": "context-pack", "version": 1 }],
            "libraryManifestHash": TOY_ZERO_HASH,
            "sourceHashRefs": [{ "ref": "source-toy", "hash": TOY_ZERO_HASH }],
            "changeSetRefs": ["change-set-toy"],
            "changeSetHash": TOY_ZERO_HASH,
            "selectionPolicyVersion": "context-pack-selection-v1",
            "promptTemplateVersion": "draft-generation-v1",
            "includedItemHashes": [{ "ref": "scene-card-toy", "hash": TOY_ZERO_HASH }],
            "activationHashes": [{ "ref": "activation-scene-toy", "hash": TOY_ZERO_HASH }],
            "value": TOY_ZERO_HASH
        },
        "libraryManifestHash": TOY_ZERO_HASH,
        "providerConfigRef": "provider-config-fake-toy",
        "providerKind": "fake-test",
        "model": "fake-deterministic-v1",
        "promptTemplateVersion": "draft-generation-v1",
        "outputSchemaRef": "toy-draft-output",
        "outputSchemaVersion": 1,
        "permissions": {
            "allowLongTermWrite": false,
            "allowFilesystemWrite": false,
            "allowShell": false,
            "allowNetwork": false,
            "allowTools": false,
            "requiresUserApproval": true
        },
        "createdBy": "user",
        "timeoutMs": 30000,
        "cancellation": { "requested": false, "gracefulTerminationAttempted": false },
        "inputSummary": "Toy context.",
        "eventRefs": []
    }))
    .expect("fixture job")
}

fn fixture_config(enabled: bool, auth_mode: ProviderAuthMode, advanced: bool) -> ProviderConfig {
    ProviderConfig {
        schema_version: 1,
        id: "provider-config-fake-toy".to_string(),
        object_type: "provider-config".to_string(),
        label: "Toy Provider Config".to_string(),
        created_at: "2026-06-06T00:00:00.000Z".to_string(),
        updated_at: "2026-06-06T00:00:00.000Z".to_string(),
        status: "active".to_string(),
        tags: vec!["toy".to_string()],
        metadata: Default::default(),
        audit_trail: vec![],
        provider_kind: job_provider_kind(),
        enabled,
        auth_mode,
        secret_ref: Some("TOY_PROVIDER_AUTH_REF".to_string()),
        base_url: None,
        default_model: "fake-deterministic-v1".to_string(),
        default_timeout_ms: 30000,
        capability_refs: vec![],
        redaction_policy_version: "provider-redaction-v1".to_string(),
        advanced_agent_mode_enabled: advanced,
        fail_closed_defaults: true,
    }
}

fn job_provider_kind() -> infinite_typewriter_providers::ProviderKind {
    infinite_typewriter_providers::ProviderKind::FakeTest
}
