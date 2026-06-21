use infinite_typewriter_core::model::character::{
    CharacterStableProfile, RelationshipEdge, RelationshipTemperature,
};

#[test]
fn character_contract_round_trips_stable_profile() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "character-practical-learner",
        "type": "character-stable-profile",
        "label": "Practical Learner",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workId": "work-public-toy",
        "displayName": "Practical Learner",
        "role": "protagonist",
        "coreDrive": "Preserve the clue with the broadest future use.",
        "coreDriveEvidenceRefIds": ["evidence-toy-pressure"],
        "fearOrWound": "Worries that early certainty hides a later cost.",
        "boundaries": ["Will not spend another person's resource without consent."],
        "boundaryEvidenceRefIds": ["evidence-toy-pressure"],
        "competencies": ["Careful observation", "Tool use"],
        "values": ["Consent", "Practical truth"],
        "voiceNotes": ["Short concrete sentences under pressure."],
        "voiceEvidenceRefIds": ["evidence-toy-pressure"],
        "actionFingerprints": ["Checks cost before acting."]
    });

    let profile: CharacterStableProfile = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(profile).unwrap(), value);
}

#[test]
fn relationship_contract_round_trips_edge() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "relationship-mentor-learner",
        "type": "relationship-edge",
        "label": "Mentor and Learner",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workId": "work-public-toy",
        "participantIds": ["character-practical-learner", "character-careful-mentor"],
        "relationshipKind": "mentor",
        "currentTemperature": "warm",
        "trust": 7,
        "tension": 3,
        "publicStatus": "Cooperative partners.",
        "privateStatus": "The learner withholds uncertainty.",
        "evidenceRefIds": ["evidence-toy-pressure"]
    });

    let edge: RelationshipEdge = serde_json::from_value(value.clone()).unwrap();
    let _: RelationshipTemperature = serde_json::from_value(serde_json::json!({
        "schemaVersion": 1,
        "id": "temperature-mentor-learner-scene",
        "type": "relationship-temperature",
        "label": "Scene temperature",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "relationshipEdgeId": "relationship-mentor-learner",
        "scopeRef": "scene-toy-gate",
        "affinity": 6,
        "conflict": 2,
        "dependency": 5,
        "recentTriggers": ["Shared token decision."]
    }))
    .unwrap();

    assert_eq!(serde_json::to_value(edge).unwrap(), value);
}

#[test]
fn relationship_contract_rejects_scores_outside_schema_range() {
    let mut value = relationship_edge_value();
    value["trust"] = serde_json::json!(11);

    let result = serde_json::from_value::<RelationshipEdge>(value);

    assert!(result.is_err());

    let mut value = relationship_edge_value();
    value["tension"] = serde_json::json!(11);

    let result = serde_json::from_value::<RelationshipEdge>(value);

    assert!(result.is_err());

    let result = serde_json::from_value::<RelationshipTemperature>(serde_json::json!({
        "schemaVersion": 1,
        "id": "temperature-mentor-learner-scene",
        "type": "relationship-temperature",
        "label": "Scene temperature",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "relationshipEdgeId": "relationship-mentor-learner",
        "scopeRef": "scene-toy-gate",
        "affinity": 11,
        "conflict": 2,
        "dependency": 5,
        "recentTriggers": ["Shared token decision."]
    }));

    assert!(result.is_err());
}

#[test]
fn relationship_contract_accepts_decimal_scores_allowed_by_schema() {
    let mut value = relationship_edge_value();
    value["trust"] = serde_json::json!(7.5);

    let edge: RelationshipEdge = serde_json::from_value(value).unwrap();
    let round_tripped = serde_json::to_value(edge).unwrap();

    assert_eq!(round_tripped["trust"], serde_json::json!(7.5));
}

#[test]
fn relationship_contract_rejects_unknown_temperature_levels() {
    let mut value = relationship_edge_value();
    value["currentTemperature"] = serde_json::json!("boiling");

    let result = serde_json::from_value::<RelationshipEdge>(value);

    assert!(result.is_err());
}

fn relationship_edge_value() -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": 1,
        "id": "relationship-mentor-learner",
        "type": "relationship-edge",
        "label": "Mentor and Learner",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workId": "work-public-toy",
        "participantIds": ["character-practical-learner", "character-careful-mentor"],
        "relationshipKind": "mentor",
        "currentTemperature": "warm",
        "trust": 7,
        "tension": 3,
        "publicStatus": "Cooperative partners.",
        "privateStatus": "The learner withholds uncertainty.",
        "evidenceRefIds": ["evidence-toy-pressure"]
    })
}
