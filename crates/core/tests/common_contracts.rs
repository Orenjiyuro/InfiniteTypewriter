use infinite_typewriter_core::model::common::{AuditTrailAction, AuditTrailActor, AuditTrailEntry};

#[test]
fn common_contract_round_trips_audit_entry() {
    let value = serde_json::json!({
        "at": "2026-06-05T00:00:00.000Z",
        "action": "create",
        "actor": "user",
        "reason": "Toy public contract creation.",
        "sourceRefIds": ["evidence-toy-pressure"]
    });

    let entry: AuditTrailEntry = serde_json::from_value(value.clone()).unwrap();

    assert_eq!(entry.action, AuditTrailAction::Create);
    assert_eq!(entry.actor, AuditTrailActor::User);
    assert_eq!(serde_json::to_value(entry).unwrap(), value);
}
