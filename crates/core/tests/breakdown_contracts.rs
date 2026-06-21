use infinite_typewriter_core::{
    BreakdownEvidenceCard, BreakdownProject, BreakdownRecipe, BreakdownReferenceMechanism,
};

#[test]
fn breakdown_contract_round_trips_recipe() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "recipe-toy-longform",
        "type": "breakdown-recipe",
        "label": "Toy Longform Recipe",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "concerns": [{
            "id": "concern-character-toy",
            "kind": "character",
            "label": "Character profile",
            "requiredForRecipe": true,
            "coverage": {
                "scope": "toy chapter",
                "unavailableScope": ["later chapters"],
                "evidenceRefIds": ["evidence-toy-pressure"],
                "confidence": "medium",
                "rationale": "The toy fixture covers one invented scene only."
            },
            "fieldEvidenceRefs": [{
                "fieldPath": "stableProfile.coreDrive",
                "evidenceRefIds": ["evidence-toy-pressure"],
                "required": true
            }],
            "transferRuleIds": ["transfer-rule-toy"]
        }],
        "transferRules": [{
            "id": "transfer-rule-toy",
            "sourceFact": "Invented public pressure fact.",
            "transferableMechanism": "A visible constraint reveals priority.",
            "targetVariableSlots": ["constraint", "choice", "cost"],
            "prohibitedResidue": ["No copied names", "No copied event sequence"],
            "misuseRisks": ["Constraint appears after the choice."]
        }],
        "modules": [{
            "id": "module-character-profile-v1",
            "label": "Character profile v1",
            "appliesToConcernIds": ["concern-character-toy"],
            "fields": [{
                "name": "coreDrive",
                "valueKind": "text",
                "evidenceRequired": true,
                "displayHint": "card",
                "migrationStrategy": "manual-review"
            }]
        }],
        "viewProjections": [{
            "id": "view-character-card-toy",
            "label": "Character card",
            "targetConcernId": "concern-character-toy",
            "sourceFieldPaths": ["stableProfile.coreDrive"],
            "displayKind": "card"
        }],
        "completionGateChecks": [{
            "id": "gate-character-profile-toy",
            "targetRef": "concern-character-toy",
            "gateLevel": "setup",
            "checkKind": "character-profile-evidence",
            "blocking": true,
            "passed": true,
            "evidenceRefIds": ["evidence-toy-pressure"],
            "notes": "Toy character concern has one evidence reference."
        }]
    });

    let recipe: BreakdownRecipe = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(recipe).unwrap(), value);
}

#[test]
fn breakdown_contract_round_trips_project() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "breakdown-project-toy",
        "type": "breakdown-project",
        "label": "Toy Breakdown Project",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "draft",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "sourceId": "source-public-toy",
        "recipeId": "recipe-toy-longform",
        "objective": "Extract public-safe mechanisms from a toy source.",
        "scopeIds": ["breakdown-scope-toy"],
        "outputRefIds": ["evidence-toy-pressure"]
    });

    let project: BreakdownProject = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(project).unwrap(), value);
}

#[test]
fn breakdown_contract_round_trips_formal_evidence_card() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "evidence-toy-pressure",
        "type": "evidence-card",
        "label": "Toy pressure evidence",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "breakdownProjectId": "breakdown-project-toy",
        "breakdownScopeId": "breakdown-scope-toy",
        "sourceLocator": "chapter-1-scene-1",
        "observationKind": "observation",
        "observedTechnique": "A visible resource limit turns route choice into pressure.",
        "functionInSource": "Creates pressure without copying events or names.",
        "transferableMechanism": "Expose a scarce resource before the character commits.",
        "prohibitedCopying": ["No names", "No dialogue", "No event sequence"],
        "coverage": {
            "scope": "toy chapter",
            "unavailableScope": ["later chapters"],
            "evidenceRefIds": ["evidence-toy-pressure"],
            "confidence": "medium",
            "rationale": "The toy fixture covers one invented scene only."
        },
        "confidence": "medium",
        "fieldEvidenceRefs": [{
            "fieldPath": "observedTechnique",
            "evidenceRefIds": ["evidence-toy-pressure"],
            "required": true
        }]
    });

    let evidence: BreakdownEvidenceCard = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(evidence).unwrap(), value);
}

#[test]
fn breakdown_contract_round_trips_formal_reference_mechanism() {
    let value = serde_json::json!({
        "schemaVersion": 1,
        "id": "reference-mechanism-toy",
        "type": "reference-mechanism",
        "label": "Toy scarcity before commitment",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "sourceEvidenceRefs": ["evidence-toy-pressure"],
        "mechanism": "Scarcity before commitment",
        "transferRuleIds": ["transfer-rule-toy"],
        "variableSlots": ["constraint", "choice", "cost"],
        "prohibitedResidue": ["No copied names", "No copied event sequence"],
        "misuseRisks": ["Constraint appears after the choice."],
        "applicableTaskTypes": ["scene-planning", "draft-generation"],
        "applicableScope": "original scenes that need costed choices",
        "currentWorkConflictChecks": ["Do not reuse if the current scene already resolved the cost."]
    });

    let mechanism: BreakdownReferenceMechanism = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(serde_json::to_value(mechanism).unwrap(), value);
}

#[test]
fn breakdown_contract_rejects_hidden_view_projection_display_kind() {
    let mut value = breakdown_recipe_value();
    value["viewProjections"][0]["displayKind"] = serde_json::json!("hidden");

    let result = serde_json::from_value::<BreakdownRecipe>(value);

    assert!(result.is_err());
}

fn breakdown_recipe_value() -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": 1,
        "id": "recipe-toy-longform",
        "type": "breakdown-recipe",
        "label": "Toy Longform Recipe",
        "createdAt": "2026-06-05T00:00:00.000Z",
        "updatedAt": "2026-06-05T00:00:00.000Z",
        "status": "active",
        "tags": ["toy"],
        "metadata": {},
        "auditTrail": [],
        "concerns": [],
        "transferRules": [],
        "modules": [],
        "viewProjections": [{
            "id": "view-character-card-toy",
            "label": "Character card",
            "targetConcernId": "concern-character-toy",
            "sourceFieldPaths": ["stableProfile.coreDrive"],
            "displayKind": "card"
        }],
        "completionGateChecks": []
    })
}
