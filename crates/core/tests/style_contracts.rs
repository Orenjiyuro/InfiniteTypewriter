use infinite_typewriter_core::model::style::{NarrativeDNA, StyleProfile};

#[test]
fn style_contract_round_trips_profile() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "style-profile-toy",
        "type": "style-profile",
        "label": "Toy Practical Pressure Style",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "ownerRef": "work-public-toy",
        "scope": "single invented scene",
        "styleDimensions": ["plain pressure", "visible cost"],
        "voiceConstraints": ["Use concrete cause and effect."],
        "doNotImitate": ["Do not copy source phrasing."],
        "evidenceRefIds": ["evidence-toy-pressure"],
        "transferRuleIds": ["transfer-rule-toy"],
        "misuseRisks": ["Pressure is stated after the action."],
        "taskUseHints": ["scene-planning", "style-revision"]
    });

    let profile: StyleProfile = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(profile).unwrap(), value);
}

#[test]
fn style_contract_round_trips_narrative_dna_v2() {
    let value = serde_json::json!({
        "schemaVersion": 2,
        "id": "narrative-dna-toy",
        "type": "narrative-dna",
        "label": "Toy Narrative DNA",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "workId": "work-public-toy",
        "readerExperienceGoal": "Make choices feel visibly costly.",
        "promiseProgressPayoffPattern": "Promise a cost, escalate it, pay it off with a choice.",
        "tensionEngine": "Visible scarcity before commitment.",
        "sceneRhythm": "Set cost, force choice, show consequence.",
        "signatureMoves": ["cost-before-choice"],
        "evidenceRefIds": ["evidence-toy-pressure"],
        "coverage": "single invented scene",
        "confidence": "medium",
        "mechanismRefIds": ["reference-mechanism-toy"],
        "transferRuleRefs": ["transfer-rule-toy"],
        "applicableTaskTypes": ["outline-generation", "draft-generation"],
        "applicableScope": "original scenes that need practical pressure",
        "misuseRisks": ["Using scarcity after the choice weakens causality."],
        "doNotCopy": ["Do not copy source scenes."],
        "prohibitedResidue": ["No copied names", "No copied event sequence"],
        "currentWorkConflictChecks": ["Skip if the current work already resolved the cost."]
    });

    let dna: NarrativeDNA = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(dna).unwrap(), value);
}
