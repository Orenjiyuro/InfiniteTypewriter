use std::collections::BTreeMap;

use serde_json::{Map, Value};

pub const REDACTION_POLICY_VERSION: &str = "provider-redaction-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionResult<T> {
    pub value: T,
    pub redaction_policy_version: &'static str,
    pub replacement_count: usize,
}

pub fn redact_headers(
    headers: &BTreeMap<String, String>,
) -> RedactionResult<BTreeMap<String, String>> {
    let mut replacement_count = 0;
    let value = headers
        .iter()
        .map(|(key, value)| {
            let redacted = if is_auth_key(key) {
                replacement_count += 1;
                "[redacted:auth]".to_string()
            } else {
                let redacted = redact_text_value(value);
                replacement_count += redacted.1;
                redacted.0
            };
            (key.clone(), redacted)
        })
        .collect();

    RedactionResult {
        value,
        redaction_policy_version: REDACTION_POLICY_VERSION,
        replacement_count,
    }
}

pub fn redact_env(env: &BTreeMap<String, String>) -> RedactionResult<BTreeMap<String, String>> {
    let mut replacement_count = 0;
    let value = env
        .iter()
        .map(|(key, value)| {
            if is_secret_key(key) {
                replacement_count += 1;
                (key.clone(), "[redacted:secret]".to_string())
            } else {
                let redacted = redact_text_value(value);
                replacement_count += redacted.1;
                (key.clone(), redacted.0)
            }
        })
        .collect();

    RedactionResult {
        value,
        redaction_policy_version: REDACTION_POLICY_VERSION,
        replacement_count,
    }
}

pub fn redact_text(input: &str) -> RedactionResult<String> {
    let (value, replacement_count) = redact_text_value(input);
    RedactionResult {
        value,
        redaction_policy_version: REDACTION_POLICY_VERSION,
        replacement_count,
    }
}

pub fn redact_json_value(input: &Value) -> RedactionResult<Value> {
    let (value, replacement_count) = redact_json_inner(None, input);
    RedactionResult {
        value,
        redaction_policy_version: REDACTION_POLICY_VERSION,
        replacement_count,
    }
}

fn redact_json_inner(parent_key: Option<&str>, input: &Value) -> (Value, usize) {
    match input {
        Value::Object(map) => {
            let mut replacements = 0;
            let redacted = map
                .iter()
                .map(|(key, value)| {
                    let (value, count) = redact_json_inner(Some(key), value);
                    replacements += count;
                    (key.clone(), value)
                })
                .collect::<Map<String, Value>>();
            (Value::Object(redacted), replacements)
        }
        Value::Array(values) => {
            let mut replacements = 0;
            let redacted = values
                .iter()
                .map(|value| {
                    let (value, count) = redact_json_inner(parent_key, value);
                    replacements += count;
                    value
                })
                .collect();
            (Value::Array(redacted), replacements)
        }
        Value::String(value) => {
            if parent_key.is_some_and(is_auth_key) {
                (Value::String("[redacted:auth]".to_string()), 1)
            } else if parent_key.is_some_and(is_secret_key) {
                (Value::String("[redacted:secret]".to_string()), 1)
            } else {
                let (redacted, count) = redact_text_value(value);
                (Value::String(redacted), count)
            }
        }
        _ => (input.clone(), 0),
    }
}

fn redact_text_value(input: &str) -> (String, usize) {
    let (value, path_count) = redact_user_paths(input);
    let (value, secret_count) = redact_secret_tokens(&value);
    (value, path_count + secret_count)
}

fn redact_user_paths(input: &str) -> (String, usize) {
    let mut output = Vec::new();
    let mut count = 0;
    for token in input.split_whitespace() {
        if token.starts_with("C:\\Users\\")
            || token.starts_with("C:/Users/")
            || token.starts_with("/Users/")
            || token.starts_with("/home/")
        {
            output.push("[redacted:user-path]");
            count += 1;
        } else {
            output.push(token);
        }
    }
    (output.join(" "), count)
}

fn redact_secret_tokens(input: &str) -> (String, usize) {
    let mut output = Vec::new();
    let mut count = 0;
    for token in input.split_whitespace() {
        let trimmed = token.trim_matches(|c: char| c == '"' || c == '\'' || c == ',' || c == ';');
        if looks_like_secret_value(trimmed) {
            output.push(token.replace(trimmed, "[redacted:secret]"));
            count += 1;
        } else {
            output.push(token.to_string());
        }
    }
    (output.join(" "), count)
}

fn is_auth_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key == "authorization" || key == "x-api-key" || key == "x-goog-api-key" || key == "api-key"
}

fn is_secret_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key.contains("secret") || key.contains("api_key") || key.contains("api-key") || key == "token"
}

fn looks_like_secret_value(value: &str) -> bool {
    value.starts_with("sk-")
        || value.contains("_secret_")
        || value.ends_with("-secret-value")
        || value.contains("secret-value")
}
