use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaVersion {
    pub schema_id: String,
    pub version: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashManifestEntry {
    pub source_object_type: String,
    pub source_ref: String,
    pub source_schema_version: u16,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextHashInput {
    pub schema_version_map: Vec<SchemaVersion>,
    pub library_manifest_hash: String,
    pub source_hash_refs: Vec<HashManifestEntry>,
    pub change_set_refs: Vec<String>,
    pub change_set_hash: String,
    pub selection_policy_version: String,
    pub included_item_hashes: Vec<HashManifestEntry>,
    pub activation_hashes: Vec<HashManifestEntry>,
    pub excluded_item_hashes: Vec<HashManifestEntry>,
}

pub fn canonical_context_hash(input: &ContextHashInput) -> String {
    let canonical = normalized_input(input);
    let bytes = serde_json::to_vec(&canonical).expect("canonical hash input must serialize");
    let digest = Sha256::digest(bytes);
    hex_lower(&digest)
}

fn normalized_input(input: &ContextHashInput) -> Value {
    let mut normalized = input.clone();
    normalized.schema_version_map.sort_by(|a, b| {
        a.schema_id
            .cmp(&b.schema_id)
            .then(a.version.cmp(&b.version))
    });
    sort_hash_entries(&mut normalized.source_hash_refs);
    normalized.change_set_refs.sort();
    sort_hash_entries(&mut normalized.included_item_hashes);
    sort_hash_entries(&mut normalized.activation_hashes);
    sort_hash_entries(&mut normalized.excluded_item_hashes);

    canonicalize_value(serde_json::to_value(normalized).expect("hash input must convert to value"))
}

fn sort_hash_entries(entries: &mut [HashManifestEntry]) {
    entries.sort_by(|a, b| {
        a.source_object_type
            .cmp(&b.source_object_type)
            .then(a.source_ref.cmp(&b.source_ref))
            .then(a.source_schema_version.cmp(&b.source_schema_version))
            .then(a.hash.cmp(&b.hash))
    });
}

fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(canonicalize_value).collect()),
        Value::Object(object) => {
            let mut canonical = Map::new();
            let mut keys: Vec<_> = object.keys().cloned().collect();
            keys.sort();
            for key in keys {
                let value = object
                    .get(&key)
                    .cloned()
                    .expect("key collected from object must exist");
                canonical.insert(key, canonicalize_value(value));
            }
            Value::Object(canonical)
        }
        scalar => scalar,
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
