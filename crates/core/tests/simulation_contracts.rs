use infinite_typewriter_core::model::simulation::{
    PuppeteeringRisk, SimulationContext, SimulationResult,
};

#[test]
fn simulation_contract_round_trips_context() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "simulation-token-choice",
        "type": "simulation-context",
        "label": "Token choice simulation",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workId": "work-public-toy",
        "taskRef": "task-character-simulation-toy",
        "sceneOrSegmentRef": "scene-toy-gate",
        "characterRefs": ["character-practical-learner"],
        "stableProfileRefs": ["character-practical-learner"],
        "dynamicStateRefs": ["dynamic-state-gate-choice"],
        "relationshipRefs": ["relationship-mentor-learner"],
        "constraints": ["Token must be spent before either route opens."],
        "question": "Which route can the character choose without violating boundaries?",
        "candidateActions": []
    });

    let context: SimulationContext = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(context).unwrap(), value);
}

#[test]
fn simulation_contract_round_trips_risk_and_result() {
    let risk_value = serde_json::json!({
        "id": "risk-forced-token-spend",
        "actionCandidateId": "action-spend-token-alone",
        "riskKind": "boundary-violation",
        "conflictingRefs": ["character-practical-learner"],
        "severity": "high",
        "explanation": "The action spends another person's resource without consent.",
        "repairOptions": ["Ask the mentor first.", "Shift the token ownership."]
    });
    let result_value = serde_json::json!({
        "schemaVersion": 1,
        "id": "simulation-result-token-choice",
        "type": "simulation-result",
        "label": "Token choice result",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "simulationContextId": "simulation-token-choice",
        "candidateRefs": ["action-spend-token-alone"],
        "recommendedDirection": "Let the character ask before spending the token.",
        "requiredUserDecisions": ["Confirm whether the token belongs to both characters."],
        "draftChangeSetRefs": [],
        "outlineAdjustmentSuggestionIds": ["outline-adjustment-token-choice"]
    });

    let risk: PuppeteeringRisk = serde_json::from_value(risk_value.clone()).unwrap();
    let result: SimulationResult = serde_json::from_value(result_value.clone()).unwrap();

    assert_eq!(serde_json::to_value(risk).unwrap(), risk_value);
    assert_eq!(serde_json::to_value(result).unwrap(), result_value);
}
