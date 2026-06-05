use infinite_typewriter_core::{validate_evidence_card_value, ContextPack};
use serde_json::json;

#[test]
fn context_pack_fixture_round_trips() {
    let fixture = public_safe_context_pack_fixture();

    let context_pack: ContextPack =
        serde_json::from_value(fixture.clone()).expect("fixture should deserialize");
    let round_tripped =
        serde_json::to_value(context_pack).expect("context pack should serialize");

    assert_eq!(round_tripped, fixture);
}

#[test]
fn evidence_card_requires_evidence_source_coverage_and_confidence() {
    let incomplete_card = json!({
        "id": "evidence-missing-required",
        "kind": "evidence-card",
        "observedTechnique": "A choice is constrained by a public, invented resource limit.",
        "sourceFunction": "Demonstrates pressure without copying a source event.",
        "transferableMechanism": "Make a constraint visible before the character chooses.",
        "forbiddenCopy": ["Do not copy named events or dialogue."],
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z"
    });

    let error =
        validate_evidence_card_value(&incomplete_card).expect_err("required fields are missing");

    assert_eq!(
        error.missing_fields(),
        &["evidence", "source", "coverage", "confidence"]
    );
}

#[test]
fn evidence_card_rejects_empty_or_incomplete_evidence_fields() {
    let fixture = public_safe_context_pack_fixture();
    let card = fixture["evidenceCards"][0].clone();

    let mut empty_evidence = card.clone();
    empty_evidence["evidence"] = json!([]);
    let error = validate_evidence_card_value(&empty_evidence)
        .expect_err("empty evidence arrays should be rejected");
    assert_eq!(error.missing_fields(), &["evidence"]);

    let mut missing_source_locator = card.clone();
    missing_source_locator["source"]["locator"] = json!("");
    let error = validate_evidence_card_value(&missing_source_locator)
        .expect_err("blank source locator should be rejected");
    assert_eq!(error.missing_fields(), &["source.locator"]);

    let mut missing_coverage_scope = card.clone();
    missing_coverage_scope["coverage"]["scope"] = json!("");
    let error = validate_evidence_card_value(&missing_coverage_scope)
        .expect_err("blank coverage scope should be rejected");
    assert_eq!(error.missing_fields(), &["coverage.scope"]);

    let mut missing_confidence_rationale = card.clone();
    missing_confidence_rationale["confidence"]["rationale"] = json!("");
    let error = validate_evidence_card_value(&missing_confidence_rationale)
        .expect_err("blank confidence rationale should be rejected");
    assert_eq!(error.missing_fields(), &["confidence.rationale"]);
}

fn public_safe_context_pack_fixture() -> serde_json::Value {
    json!({
        "id": "context-pack-toy",
        "kind": "context-pack",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "purpose": "Toy original planning context",
        "evidenceCards": [{
            "id": "evidence-toy-pressure",
            "kind": "evidence-card",
            "source": {
                "sourceId": "source-public-toy",
                "title": "Public Toy Source",
                "locator": "chapter-1-scene-1"
            },
            "evidence": [{
                "locator": "chapter-1-scene-1",
                "summary": "Invented public example: a gate opens only after a limited token is spent."
            }],
            "observedTechnique": "A visible resource limit turns a simple route into a choice.",
            "sourceFunction": "Creates pressure without needing copied events, names, or dialogue.",
            "transferableMechanism": "Expose a scarce resource before the character commits to a route.",
            "forbiddenCopy": ["Do not copy source events, names, dialogue, or relationship chains."],
            "coverage": {
                "scope": "single-scene",
                "notes": "Covers only the pressure mechanism in a toy scene."
            },
            "confidence": {
                "level": "medium",
                "rationale": "The example is narrow and intentionally public-safe."
            },
            "createdAt": "2026-06-05T00:00:00.000Z",
            "updatedAt": "2026-06-05T00:00:00.000Z"
        }],
        "readerContracts": [{
            "id": "reader-contract-toy",
            "kind": "reader-contract",
            "promise": "A practical problem will reveal character priorities.",
            "progress": "Each choice shows a cost.",
            "payoff": "The final route resolves the immediate constraint.",
            "tone": "focused",
            "emotionalTarget": "curiosity",
            "evidenceCardIds": ["evidence-toy-pressure"]
        }],
        "knowledgeBoundaries": [{
            "id": "knowledge-boundary-toy",
            "kind": "knowledge-boundary",
            "pov": "third-person-limited",
            "narratorKnows": ["The current room condition."],
            "readerKnows": ["A token is required."],
            "withheldInformation": ["The later consequence of spending the token."],
            "unreliable": false
        }],
        "settingPressures": [{
            "id": "setting-pressure-toy",
            "kind": "setting-pressure",
            "settingElement": "Token gate",
            "restriction": "Only one path can be opened.",
            "resource": "One-use token",
            "cost": "The alternate path closes.",
            "choiceImpact": "The protagonist must choose which clue to preserve."
        }],
        "referenceMechanisms": [{
            "id": "reference-mechanism-toy",
            "kind": "reference-mechanism",
            "evidenceCardId": "evidence-toy-pressure",
            "mechanism": "Scarcity before commitment",
            "transferRules": ["Keep the scarcity abstract.", "Invent new setting details."],
            "forbiddenCopies": ["Names", "Dialogue", "Event sequence"]
        }],
        "sceneCards": [{
            "id": "scene-card-toy",
            "kind": "scene-card",
            "workId": "work-public-toy",
            "title": "Toy Gate Choice",
            "pov": "third-person-limited",
            "knowledgeBoundaryId": "knowledge-boundary-toy",
            "settingPressureIds": ["setting-pressure-toy"],
            "readerContractId": "reader-contract-toy",
            "characterGoals": ["Preserve the most useful clue."],
            "conflictAgenda": "Resource scarcity forces prioritization.",
            "revealedInformation": ["The gate consumes the token."],
            "hiddenInformation": ["The closed path still matters later."],
            "stateChange": "The protagonist commits to one path.",
            "turn": "A tool becomes a cost."
        }],
        "draftReviewItems": [{
            "id": "draft-review-toy",
            "kind": "draft-review-item",
            "targetId": "scene-card-toy",
            "severity": "medium",
            "category": "continuity",
            "finding": "Clarify that the token is consumed.",
            "recommendation": "Add one original sentence before the gate opens.",
            "evidenceCardIds": ["evidence-toy-pressure"]
        }],
        "changeSets": [{
            "id": "change-set-toy",
            "kind": "change-set",
            "createdAt": "2026-06-05T00:00:00.000Z",
            "summary": "Clarified the toy gate cost.",
            "changes": [{
                "targetId": "scene-card-toy",
                "changeType": "update",
                "description": "Make the cost explicit before the choice."
            }],
            "affectedIds": ["scene-card-toy", "setting-pressure-toy"]
        }]
    })
}
