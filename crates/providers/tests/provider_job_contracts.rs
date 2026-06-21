use infinite_typewriter_providers::ProviderJob;
use serde_json::json;

const TOY_ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn provider_job_round_trips_public_safe_fixture() {
    let fixture = json!({
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
            "schemaVersionMap": [{
                "schemaId": "https://infinite-typewriter.local/schemas/context-pack.schema.json",
                "version": 1
            }],
            "libraryManifestHash": TOY_ZERO_HASH,
            "sourceHashRefs": [{ "ref": "source-toy", "hash": TOY_ZERO_HASH }],
            "changeSetRefs": ["change-set-toy"],
            "changeSetHash": TOY_ZERO_HASH,
            "selectionPolicyVersion": "context-pack-selection-v1",
            "promptTemplateVersion": "draft-generation-v1",
            "includedItemHashes": [{ "ref": "scene-card-toy", "hash": TOY_ZERO_HASH }],
            "activationHashes": [{ "ref": "activation-scene-toy", "hash": TOY_ZERO_HASH }],
            "excludedItemHashes": [{ "ref": "exclusion-manifest-toy", "hash": TOY_ZERO_HASH }],
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
        "startedAt": "2026-06-06T00:00:00.000Z",
        "endedAt": "2026-06-06T00:00:00.000Z",
        "timeoutMs": 30000,
        "cancellation": {
            "requested": false,
            "gracefulTerminationAttempted": false
        },
        "inputSummary": "Toy draft-generation context pack with one selected scene manifest.",
        "outputSummary": "Deterministic fake provider draft summary.",
        "draftOutputRef": "draft-output-fake-toy",
        "usage": {
            "providerKind": "fake-test",
            "model": "fake-deterministic-v1",
            "inputTokens": 12,
            "outputTokens": 8,
            "totalTokens": 20,
            "durationMs": 1,
            "requestId": "fake-request-toy",
            "usageRecordedAt": "2026-06-06T00:00:00.000Z",
            "isBillable": false
        },
        "eventRefs": ["event-job-created-toy", "event-usage-recorded-toy"]
    });

    let job: ProviderJob = serde_json::from_value(fixture.clone()).expect("fixture deserializes");
    let serialized = serde_json::to_value(job).expect("fixture serializes");

    assert_eq!(serialized, fixture);
}

#[test]
fn creative_discussion_job_round_trips_as_freeform_draft() {
    let fixture = json!({
        "schemaVersion": 1,
        "id": "provider-job-discussion-toy",
        "type": "provider-job",
        "label": "Toy Creative Discussion Job",
        "createdAt": "2026-06-06T00:00:00.000Z",
        "updatedAt": "2026-06-06T00:00:00.000Z",
        "status": "ready-for-provider",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "taskType": "creative-discussion",
        "targetRef": "work-public-toy",
        "contextPackRef": "context-pack-discussion-toy",
        "contextHash": {
            "algorithm": "sha256",
            "canonicalization": "json-canonical-v1",
            "schemaVersionMap": [{ "schemaId": "context-pack", "version": 1 }],
            "libraryManifestHash": TOY_ZERO_HASH,
            "sourceHashRefs": [],
            "changeSetRefs": [],
            "changeSetHash": TOY_ZERO_HASH,
            "selectionPolicyVersion": "context-pack-selection-v1",
            "promptTemplateVersion": "creative-discussion-v1",
            "includedItemHashes": [],
            "activationHashes": [],
            "value": TOY_ZERO_HASH
        },
        "libraryManifestHash": TOY_ZERO_HASH,
        "providerConfigRef": "provider-config-fake-toy",
        "providerKind": "fake-test",
        "model": "fake-deterministic-v1",
        "promptTemplateVersion": "creative-discussion-v1",
        "outputSchemaRef": "freeform-discussion-draft-v1",
        "outputSchemaVersion": 1,
        "outputMode": "freeform-draft",
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
        "inputSummary": "Toy creative discussion context.",
        "draftOutputRef": "discussion-draft-output-toy",
        "eventRefs": []
    });

    let job: ProviderJob = serde_json::from_value(fixture.clone()).expect("fixture deserializes");
    let serialized = serde_json::to_value(job).expect("fixture serializes");

    assert_eq!(serialized, fixture);
}
