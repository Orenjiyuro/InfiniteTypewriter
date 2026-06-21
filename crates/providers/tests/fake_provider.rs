use infinite_typewriter_providers::fake::{run_fake_provider, FakeProviderScenario};
use infinite_typewriter_providers::{ProviderJob, ProviderJobEventType, ProviderJobStatus};
use serde_json::json;

const TOY_ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn fake_success_records_lifecycle_draft_and_non_billable_usage() {
    let result = run_fake_provider(&fixture_job(), FakeProviderScenario::Success);

    assert_eq!(result.final_status, ProviderJobStatus::Succeeded);
    assert_eq!(
        result
            .events
            .iter()
            .map(|event| event.event_type.clone())
            .collect::<Vec<_>>(),
        vec![
            ProviderJobEventType::ProviderStarted,
            ProviderJobEventType::ProviderFinished,
            ProviderJobEventType::DraftCreated,
            ProviderJobEventType::UsageRecorded,
        ]
    );
    assert_eq!(result.usage.expect("usage").is_billable, false);
    assert_eq!(
        result.draft_output_ref.as_deref(),
        Some("draft-output-provider-job-fake-toy")
    );
}

#[test]
fn fake_error_timeout_cancel_and_approval_scenarios_are_visible() {
    let error = run_fake_provider(&fixture_job(), FakeProviderScenario::ProviderError);
    assert_eq!(error.final_status, ProviderJobStatus::Failed);
    assert!(error
        .events
        .iter()
        .any(|event| event.event_type == ProviderJobEventType::ErrorRecorded));

    let timeout = run_fake_provider(&fixture_job(), FakeProviderScenario::Timeout);
    assert_eq!(timeout.final_status, ProviderJobStatus::Failed);
    assert!(timeout
        .events
        .iter()
        .any(|event| event.event_type == ProviderJobEventType::Timeout));

    let cancelled = run_fake_provider(&fixture_job(), FakeProviderScenario::Cancelled);
    assert_eq!(cancelled.final_status, ProviderJobStatus::Cancelled);
    assert_eq!(
        cancelled.events[0].event_type,
        ProviderJobEventType::CancelRequested
    );
    assert_eq!(
        cancelled.events[1].event_type,
        ProviderJobEventType::Cancelled
    );

    let approval = run_fake_provider(&fixture_job(), FakeProviderScenario::AwaitingApproval);
    assert_eq!(approval.final_status, ProviderJobStatus::AwaitingApproval);
    assert_eq!(
        approval.events[0].event_type,
        ProviderJobEventType::ApprovalRequested
    );
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
