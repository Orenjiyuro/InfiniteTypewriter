pub mod library;
pub mod model;

pub use library::{initialize_empty_library, LibraryError};
pub use model::methodology::{
    validate_evidence_card_value, ChangeSet, ContextPack, DraftReviewItem, EvidenceCard,
    KnowledgeBoundary, ReaderContract, ReferenceMechanism, SceneCard, SettingPressure,
};
pub use model::{
    create_empty_manifest, AnalysisRecord, LibraryManifest, LibraryRoot, SourceRecord, WorkRecord,
};
