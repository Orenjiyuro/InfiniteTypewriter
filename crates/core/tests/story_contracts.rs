use infinite_typewriter_core::model::story::{
    ChangeSet, ProseQualityPass, RevisionRequest, SceneCard, Work,
};
use infinite_typewriter_core::{ForeshadowingEntry, StorySceneCard};

#[test]
fn story_contract_round_trips_work() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "work-public-toy",
        "type": "work",
        "label": "Public Toy Work",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workKind": "novel",
        "premise": "A learner must choose which clue to preserve under a visible cost.",
        "readerContractRefs": ["reader-contract-toy"],
        "outline": {
            "id": "outline-public-toy",
            "workId": "work-public-toy",
            "volumes": []
        }
    });

    let work: Work = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(work).unwrap(), value);
}

#[test]
fn story_contract_round_trips_scene_with_beat() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "scene-toy-gate",
        "type": "scene-card",
        "label": "Toy Gate Choice",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "chapterId": "chapter-toy-one",
        "sceneGoal": "Force a costed choice.",
        "povMode": "third-person-limited",
        "knowledgeBoundaryRef": "knowledge-boundary-toy",
        "settingPressureText": "A token opens only one route.",
        "entryState": "The clue path is unknown.",
        "exitState": "One route is chosen and one is closed.",
        "characterStateRefs": ["dynamic-state-gate-choice"],
        "relationshipRefs": ["relationship-mentor-learner"],
        "foreshadowingRefs": ["foreshadowing-token-cost"],
        "styleTargetRefs": ["style-profile-toy"],
        "beats": [{
            "id": "beat-token-decision",
            "sceneId": "scene-toy-gate",
            "sequence": 1,
            "beatKind": "decision",
            "intent": "Make the resource cost explicit.",
            "expectedChange": "The protagonist commits to one route.",
            "characterRefs": ["character-practical-learner"],
            "evidenceRefIds": ["evidence-toy-pressure"]
        }]
    });

    let scene: SceneCard = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(scene).unwrap(), value);
}

#[test]
fn story_contract_exports_scene_alias_and_foreshadowing_entry_from_crate_root() {
    let scene_value = serde_json::json!({
        "schemaVersion": 1,
        "id": "scene-toy-gate",
        "type": "scene-card",
        "label": "Toy Gate Choice",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "chapterId": "chapter-toy-one",
        "sceneGoal": "Force a costed choice.",
        "povMode": "third-person-limited",
        "knowledgeBoundaryRef": "knowledge-boundary-toy",
        "settingPressureText": "A token opens only one route.",
        "entryState": "The clue path is unknown.",
        "exitState": "One route is chosen and one is closed.",
        "characterStateRefs": ["dynamic-state-gate-choice"],
        "relationshipRefs": ["relationship-mentor-learner"],
        "foreshadowingRefs": ["foreshadowing-token-cost"],
        "styleTargetRefs": ["style-profile-toy"],
        "beats": []
    });
    let entry_value = serde_json::json!({
        "id": "foreshadowing-token-cost",
        "ledgerId": "foreshadowing-ledger-toy",
        "signal": "The token closes one route.",
        "plantedAtRef": "scene-toy-gate",
        "intendedPayoff": "The closed route matters later.",
        "currentState": "planted",
        "visibility": "subtle",
        "riskNotes": ["Keep the signal original."]
    });

    let _: StorySceneCard = serde_json::from_value(scene_value).unwrap();
    let _: ForeshadowingEntry = serde_json::from_value(entry_value).unwrap();
}

#[test]
fn story_contract_round_trips_revision_request() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "revision-request-toy",
        "type": "revision-request",
        "label": "Toy revision request",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "targetRef": "evidence-toy-pressure",
        "originalChunkRef": "source-chunk-toy",
        "originalDraftRef": "draft-output-fake-toy",
        "userFeedback": "Clarify the cost before the choice.",
        "outputSchemaRef": "toy-draft-output",
        "outputSchemaVersion": 1,
        "contextHash": "0000000000000000000000000000000000000000000000000000000000000000",
        "requestedRerunTaskType": "source-breakdown",
        "requestStatus": "ready-for-rerun"
    });

    let request: RevisionRequest = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(request).unwrap(), value);
}

#[test]
fn story_contract_round_trips_quality_pass_and_change_set() {
    let pass_value = serde_json::json!({
        "schemaVersion": 1,
        "id": "prose-quality-pass-toy",
        "type": "prose-quality-pass",
        "label": "Toy prose quality pass",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "targetRef": "scene-toy-gate",
        "qualityGoals": ["clarity", "cause-before-effect"],
        "beforeRef": "draft-before-toy",
        "suggestions": [{
            "targetRef": "draft-before-toy",
            "suggestedTextRef": "draft-after-toy",
            "reason": "Improve clarity before the costed choice.",
            "evidenceRefIds": ["evidence-toy-pressure"],
            "riskNotes": ["Do not imitate source phrasing."]
        }],
        "decision": "pending-user-review"
    });
    let change_set_value = serde_json::json!({
        "schemaVersion": 1,
        "id": "change-set-toy",
        "type": "change-set",
        "label": "Toy change set",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "summary": "Clarify the toy gate cost.",
        "reason": "User accepted a revision suggestion.",
        "sourceDraftRef": "draft-output-fake-toy",
        "revisionRequestRefs": ["revision-request-toy"],
        "targetRefs": ["scene-toy-gate"],
        "changes": [{
            "targetRef": "scene-toy-gate",
            "changeType": "update",
            "description": "Make the cost explicit before the choice."
        }],
        "impactScope": ["scene"],
        "confirmationState": "pending-user-confirmation"
    });

    let pass: ProseQualityPass = serde_json::from_value(pass_value.clone()).unwrap();
    let change_set: ChangeSet = serde_json::from_value(change_set_value.clone()).unwrap();

    assert_eq!(serde_json::to_value(pass).unwrap(), pass_value);
    assert_eq!(serde_json::to_value(change_set).unwrap(), change_set_value);
}
