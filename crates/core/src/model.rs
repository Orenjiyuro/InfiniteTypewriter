use serde::{Deserialize, Serialize};

pub mod methodology;

pub type IsoDateTime = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryRoot {
    pub id: String,
    pub label: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRecord {
    pub id: String,
    pub kind: SourceRecordKind,
    pub title: String,
    pub content_hash: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceRecordKind {
    Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkRecord {
    pub id: String,
    pub kind: WorkRecordKind,
    pub title: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkRecordKind {
    Work,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisRecord {
    pub id: String,
    pub kind: AnalysisRecordKind,
    pub source_id: String,
    pub title: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisRecordKind {
    Analysis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryManifest {
    pub schema_version: u16,
    pub root: LibraryRoot,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub sources: Vec<SourceRecord>,
    pub works: Vec<WorkRecord>,
    pub analyses: Vec<AnalysisRecord>,
}

pub fn create_empty_manifest(
    root: LibraryRoot,
    timestamp: impl Into<IsoDateTime>,
) -> LibraryManifest {
    let timestamp = timestamp.into();

    LibraryManifest {
        schema_version: 1,
        root,
        created_at: timestamp.clone(),
        updated_at: timestamp,
        sources: Vec::new(),
        works: Vec::new(),
        analyses: Vec::new(),
    }
}
