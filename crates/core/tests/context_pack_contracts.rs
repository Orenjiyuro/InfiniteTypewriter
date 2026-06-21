use infinite_typewriter_core::ContextPack;
use serde_json::json;

#[test]
fn canonical_context_pack_fixture_round_trips() {
    let fixture = public_safe_context_pack_fixture();

    let context_pack: ContextPack =
        serde_json::from_value(fixture.clone()).expect("fixture should deserialize");
    let round_tripped = serde_json::to_value(context_pack).expect("context pack should serialize");

    assert_eq!(round_tripped, fixture);
}

#[test]
fn source_breakdown_revision_descriptor_round_trips() {
    let fixture = serde_json::json!({
        "schemaVersion": 1,
        "id": "context-pack-breakdown-rerun-toy",
        "type": "context-pack",
        "label": "Toy Breakdown Revision Context",
        "createdAt": "2026-06-06T00:00:00.000Z",
        "updatedAt": "2026-06-06T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "taskType": "source-breakdown",
        "targetRef": "evidence-toy-pressure",
        "taskDescriptor": {
            "taskType": "source-breakdown",
            "sourceBreakdownStage": "draft-review-rerun",
            "sourceChunkRefs": ["source-chunk-toy"],
            "originalChunkRef": "source-chunk-toy",
            "originalDraftRef": "draft-output-fake-toy",
            "revisionRequestRef": "revision-request-toy",
            "userFeedbackRef": "user-feedback-toy",
            "outputSchemaRef": "toy-draft-output",
            "outputSchemaVersion": 1,
            "contextHash": "0000000000000000000000000000000000000000000000000000000000000000"
        },
        "activations": [],
        "includedItems": [],
        "exclusions": [],
        "budget": {
            "id": "budget-breakdown-rerun-toy",
            "contextPackId": "context-pack-breakdown-rerun-toy",
            "tokenBudget": {"maxInputTokens": 1200, "reservedOutputTokens": 600, "estimationMethod": "toy"},
            "fragmentBudget": {"maxFragments": 4, "maxFragmentChars": 800},
            "objectBudget": [],
            "priorityBands": {"required": 1, "high": 1, "normal": 1, "low": 0},
            "trimmingStrategy": "drop-low-priority",
            "excludedObjectTypes": [],
            "excludedRefs": [],
            "overflowPolicy": "fail-closed"
        },
        "contextHash": {
            "algorithm": "sha256",
            "canonicalization": "json-canonical-v1",
            "schemaVersionMap": [{"schemaId": "context-pack", "version": 1}],
            "libraryManifestHash": "0000000000000000000000000000000000000000000000000000000000000000",
            "sourceHashRefs": [],
            "changeSetRefs": [],
            "changeSetHash": "0000000000000000000000000000000000000000000000000000000000000000",
            "selectionPolicyVersion": "context-pack-selection-v1",
            "includedItemHashes": [],
            "activationHashes": [],
            "value": "0000000000000000000000000000000000000000000000000000000000000000"
        },
        "createdForTaskRef": "task-breakdown-rerun-toy"
    });

    let context_pack: ContextPack = serde_json::from_value(fixture.clone()).unwrap();

    assert_eq!(serde_json::to_value(context_pack).unwrap(), fixture);
}

fn public_safe_context_pack_fixture() -> serde_json::Value {
    json!({
        "schemaVersion": 1,
        "id": "context-pack-draft-toy",
        "type": "context-pack",
        "label": "Toy Draft Generation Context",
        "createdAt": "2026-06-06T00:00:00.000Z",
        "updatedAt": "2026-06-06T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "taskType": "draft-generation",
        "targetRef": "scene-card-toy",
        "activations": [{
            "id": "activation-scene-toy",
            "contextPackId": "context-pack-draft-toy",
            "sourceRef": "scene-card-toy",
            "sourceObjectType": "SceneCard",
            "sourceSchemaVersion": 1,
            "activationReason": "target-match",
            "evidenceChain": [{
                "ref": "scene-card-toy",
                "relation": "draft-context",
                "note": "Toy public fixture links the target scene to required draft context."
            }],
            "relevance": "high",
            "confidence": "high",
            "priority": "required",
            "freshnessState": "current",
            "userSelection": "auto",
            "selectedAt": "2026-06-06T00:00:00.000Z"
        }],
        "includedItems": [{
            "sourceRef": "scene-card-toy",
            "sourceObjectType": "SceneCard",
            "sourceSchemaVersion": 1,
            "contentHash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "activationRef": "activation-scene-toy"
        }],
        "exclusions": [{
            "id": "exclusion-style-budget-toy",
            "contextPackId": "context-pack-draft-toy",
            "excludedRef": "style-profile-extra-toy",
            "sourceObjectType": "StyleProfile",
            "reason": "budget-insufficient",
            "riskKind": "budget-overflow",
            "explanation": "Optional style profile exceeded the object budget for draft preview.",
            "candidateScore": "medium",
            "wouldFitIfBudget": true,
            "createdAt": "2026-06-06T00:00:00.000Z"
        }],
        "budget": {
            "id": "budget-draft-toy",
            "contextPackId": "context-pack-draft-toy",
            "tokenBudget": {
                "maxInputTokens": 1200,
                "reservedOutputTokens": 600,
                "estimationMethod": "toy-char-estimate"
            },
            "fragmentBudget": {
                "maxFragments": 6,
                "maxFragmentChars": 800
            },
            "objectBudget": [{
                "objectType": "SceneCard",
                "maxObjects": 1
            }],
            "priorityBands": {
                "required": 1,
                "high": 3,
                "normal": 2,
                "low": 0
            },
            "trimmingStrategy": "drop-low-priority",
            "excludedObjectTypes": [],
            "excludedRefs": [],
            "overflowPolicy": "fail-closed"
        },
        "contextHash": {
            "algorithm": "sha256",
            "canonicalization": "json-canonical-v1",
            "schemaVersionMap": [{
                "schemaId": "https://infinite-typewriter.local/schemas/context-pack.schema.json",
                "version": 1
            }],
            "libraryManifestHash": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            "sourceHashRefs": [{
                "ref": "source-toy",
                "hash": "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
            }],
            "changeSetRefs": ["change-set-toy"],
            "changeSetHash": "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "selectionPolicyVersion": "context-pack-selection-v1",
            "includedItemHashes": [{
                "ref": "scene-card-toy",
                "hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }],
            "activationHashes": [{
                "ref": "scene-card-toy",
                "hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }],
            "excludedItemHashes": [{
                "ref": "exclusion-manifest-toy",
                "hash": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
            }],
            "value": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        },
        "createdForTaskRef": "task-draft-preview-toy"
    })
}
