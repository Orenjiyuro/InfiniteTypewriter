use std::collections::BTreeMap;

use infinite_typewriter_providers::redaction::{
    redact_env, redact_headers, redact_json_value, redact_text, REDACTION_POLICY_VERSION,
};
use serde_json::json;

#[test]
fn redacts_auth_headers_and_secret_like_values() {
    let headers = BTreeMap::from([
        (
            "authorization".to_string(),
            "Bearer sk-test-secret-value".to_string(),
        ),
        ("content-type".to_string(), "application/json".to_string()),
        ("x-api-key".to_string(), "gemini-secret-value".to_string()),
    ]);

    let redacted = redact_headers(&headers);

    assert_eq!(redacted.value["authorization"], "[redacted:auth]");
    assert_eq!(redacted.value["x-api-key"], "[redacted:auth]");
    assert_eq!(redacted.value["content-type"], "application/json");
    assert_eq!(redacted.redaction_policy_version, REDACTION_POLICY_VERSION);
    assert_eq!(redacted.replacement_count, 2);
}

#[test]
fn redacts_env_values_text_paths_and_nested_json() {
    let env = BTreeMap::from([
        (
            "OPENAI_AUTH_REF".to_string(),
            "sk-test-secret-value".to_string(),
        ),
        ("SAFE_FLAG".to_string(), "true".to_string()),
    ]);
    let env_summary = redact_env(&env);
    assert_eq!(env_summary.value["OPENAI_AUTH_REF"], "[redacted:secret]");
    assert_eq!(env_summary.value["SAFE_FLAG"], "true");

    let text = redact_text(
        "stderr from C:\\Users\\writer\\library and /home/writer/library with token sk-test-secret-value",
    );
    assert!(text.value.contains("[redacted:user-path]"));
    assert!(text.value.contains("[redacted:secret]"));
    assert!(text.replacement_count >= 3);

    let nested = redact_json_value(&json!({
        "request": {
            "headers": {
                "Authorization": "Bearer sk-test-secret-value"
            },
            "payload": {
                "message": "path /Users/writer/library",
                "token": "sk-test-secret-value"
            }
        }
    }));

    assert_eq!(
        nested.value["request"]["headers"]["Authorization"],
        "[redacted:auth]"
    );
    assert_eq!(
        nested.value["request"]["payload"]["token"],
        "[redacted:secret]"
    );
    assert!(nested.value["request"]["payload"]["message"]
        .as_str()
        .expect("redacted string")
        .contains("[redacted:user-path]"));
    assert!(nested.replacement_count >= 3);
}
