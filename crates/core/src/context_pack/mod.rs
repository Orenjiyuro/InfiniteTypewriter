use std::collections::{BTreeMap, HashMap};

use crate::context_pack::hash::{
    canonical_context_hash, ContextHashInput, HashManifestEntry as HashInputManifestEntry,
    SchemaVersion,
};
use crate::model::common::{
    AuditTrailEntry, BaseObject, IsoDateTime, MetadataValue, ObjectStatus, StableId,
};
use crate::model::context_pack::{
    ActivationReason, Canonicalization, ConfidenceLevel, ContextActivation, ContextBudget,
    ContextExclusion, ContextExclusionReason, ContextExclusionRiskKind, ContextHash,
    ContextIncludedItem, ContextPack, ContextPriority, ContextTaskType, EvidenceChainEntry,
    FragmentBudget, FreshnessState, HashAlgorithm, ManifestHashEntry, ObjectBudget, OverflowPolicy,
    PriorityBands, RelevanceBand, SchemaVersionEntry, TokenBudget, TrimmingStrategy,
    UserSelectionState,
};

pub mod hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidatePriority {
    Required,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateObject {
    pub source_ref: StableId,
    pub source_object_type: String,
    pub source_schema_version: u16,
    pub content_hash: String,
    pub applicable_task_types: Vec<ContextTaskType>,
    pub required: bool,
    pub priority: CandidatePriority,
    pub freshness_state: FreshnessState,
    pub has_sufficient_evidence: bool,
    pub permission_allowed: bool,
    pub source_boundary_allowed: bool,
    pub user_excluded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectBudgetLimit {
    pub object_type: String,
    pub max_objects: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetRequest {
    pub max_included_items: Option<usize>,
    pub object_limits: Vec<ObjectBudgetLimit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmedChangeSetImpact {
    pub source_ref: StableId,
    pub superseded_by_ref: StableId,
    pub change_set_ref: StableId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildContextPackRequest {
    pub pack_id: StableId,
    pub label: String,
    pub created_at: IsoDateTime,
    pub task_type: ContextTaskType,
    pub target_ref: StableId,
    pub created_for_task_ref: StableId,
    pub library_manifest_hash: String,
    pub source_hash_refs: Vec<(StableId, String)>,
    pub change_set_refs: Vec<StableId>,
    pub change_set_hash: String,
    pub selection_policy_version: String,
    pub confirmed_change_set_impacts: Vec<ConfirmedChangeSetImpact>,
    pub budget: BudgetRequest,
    pub candidates: Vec<CandidateObject>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildContextPackError {
    RequiredBudgetOverflow {
        source_ref: StableId,
    },
    RequiredCandidateExcluded {
        source_ref: StableId,
        reason: ContextExclusionReason,
    },
}

pub fn build_context_pack(
    request: BuildContextPackRequest,
) -> Result<ContextPack, BuildContextPackError> {
    let mut selected: Vec<(usize, CandidateObject)> = Vec::new();
    let mut exclusions = Vec::new();

    for (index, candidate) in request.candidates.iter().cloned().enumerate() {
        let candidate = apply_change_set_freshness(&request, candidate);
        if !candidate.permission_allowed {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::PermissionBoundary,
                ContextExclusionRiskKind::PermissionDenied,
                "Candidate is outside the permitted context boundary.",
                &mut exclusions,
            )?;
            continue;
        }
        if !candidate.source_boundary_allowed {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::SourceBoundary,
                ContextExclusionRiskKind::CopyrightBoundary,
                "Candidate would cross the source boundary for this public-safe context.",
                &mut exclusions,
            )?;
            continue;
        }
        if candidate.user_excluded {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::UserExcluded,
                ContextExclusionRiskKind::UserChoice,
                "User excluded this candidate from the preview pack.",
                &mut exclusions,
            )?;
            continue;
        }
        if !candidate
            .applicable_task_types
            .iter()
            .any(|task_type| task_type == &request.task_type)
        {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::TaskIrrelevant,
                ContextExclusionRiskKind::Irrelevant,
                "Candidate is not relevant to this task type.",
                &mut exclusions,
            )?;
            continue;
        }
        if !candidate.has_sufficient_evidence {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::InsufficientEvidence,
                ContextExclusionRiskKind::LowConfidence,
                "Candidate lacks enough evidence for deterministic activation.",
                &mut exclusions,
            )?;
            continue;
        }
        if !allows_stale(&request.task_type) && is_stale(&candidate.freshness_state) {
            reject_or_exclude(
                &request,
                &candidate,
                ContextExclusionReason::StaleOrSuperseded,
                ContextExclusionRiskKind::StaleState,
                "Candidate is stale or superseded for this task type.",
                &mut exclusions,
            )?;
            continue;
        }
        selected.push((index, candidate));
    }

    selected.sort_by(|(left_index, left), (right_index, right)| {
        priority_rank(&left.priority)
            .cmp(&priority_rank(&right.priority))
            .then(left_index.cmp(right_index))
    });

    let mut included: Vec<CandidateObject> = Vec::new();
    let mut counts_by_type: HashMap<String, usize> = HashMap::new();
    let max_included = request.budget.max_included_items.unwrap_or(usize::MAX);

    for (_, candidate) in selected {
        let object_limit = request
            .budget
            .object_limits
            .iter()
            .find(|limit| limit.object_type == candidate.source_object_type)
            .map(|limit| limit.max_objects)
            .unwrap_or(usize::MAX);
        let current_type_count = counts_by_type
            .get(&candidate.source_object_type)
            .copied()
            .unwrap_or_default();
        let over_type_limit = current_type_count >= object_limit;
        let over_pack_limit = included.len() >= max_included;

        if over_type_limit || over_pack_limit {
            if candidate.required {
                return Err(BuildContextPackError::RequiredBudgetOverflow {
                    source_ref: candidate.source_ref,
                });
            }
            exclusions.push(exclusion(
                &request,
                &candidate,
                ContextExclusionReason::BudgetInsufficient,
                ContextExclusionRiskKind::BudgetOverflow,
                "Optional candidate was dropped by the context budget.",
            ));
            continue;
        }

        *counts_by_type
            .entry(candidate.source_object_type.clone())
            .or_default() += 1;
        included.push(candidate);
    }

    let activations: Vec<_> = included
        .iter()
        .map(|candidate| activation(&request, candidate))
        .collect();
    let included_items: Vec<_> = included
        .iter()
        .map(|candidate| included_item(candidate))
        .collect();
    let context_hash = context_hash(&request, &included, &exclusions, &activations);

    Ok(ContextPack {
        base: BaseObject {
            schema_version: 1,
            id: request.pack_id.clone(),
            object_type: "context-pack".to_string(),
            label: request.label.clone(),
            created_at: request.created_at.clone(),
            updated_at: request.created_at.clone(),
            status: ObjectStatus::Draft,
            tags: vec!["toy".to_string()],
            metadata: BTreeMap::<String, MetadataValue>::new(),
            audit_trail: Vec::<AuditTrailEntry>::new(),
            context_hints: None,
        },
        task_type: request.task_type.clone(),
        task_descriptor: None,
        target_ref: request.target_ref.clone(),
        activations,
        included_items,
        exclusions,
        budget: context_budget(&request),
        context_hash,
        created_for_task_ref: request.created_for_task_ref.clone(),
        created_for_job_ref: None,
    })
}

fn allows_stale(task_type: &ContextTaskType) -> bool {
    matches!(task_type, ContextTaskType::ContinuityAudit)
}

fn is_stale(freshness_state: &FreshnessState) -> bool {
    matches!(
        freshness_state,
        FreshnessState::Stale | FreshnessState::Superseded
    )
}

fn apply_change_set_freshness(
    request: &BuildContextPackRequest,
    mut candidate: CandidateObject,
) -> CandidateObject {
    if request
        .confirmed_change_set_impacts
        .iter()
        .any(|impact| impact.source_ref == candidate.source_ref)
    {
        candidate.freshness_state = FreshnessState::Superseded;
    }
    candidate
}

fn priority_rank(priority: &CandidatePriority) -> u8 {
    match priority {
        CandidatePriority::Required => 0,
        CandidatePriority::High => 1,
        CandidatePriority::Normal => 2,
        CandidatePriority::Low => 3,
    }
}

fn activation(request: &BuildContextPackRequest, candidate: &CandidateObject) -> ContextActivation {
    ContextActivation {
        id: activation_id(&candidate.source_ref),
        context_pack_id: request.pack_id.clone(),
        source_ref: candidate.source_ref.clone(),
        source_object_type: candidate.source_object_type.clone(),
        source_schema_version: candidate.source_schema_version,
        activation_reason: activation_reason(request, candidate),
        evidence_chain: vec![EvidenceChainEntry {
            r#ref: request.target_ref.clone(),
            relation: "task-context".to_string(),
            note: format!(
                "{} was selected for {}.",
                candidate.source_object_type,
                task_type_value(&request.task_type)
            ),
        }],
        relevance: match candidate.priority {
            CandidatePriority::Required | CandidatePriority::High => RelevanceBand::High,
            CandidatePriority::Normal => RelevanceBand::Medium,
            CandidatePriority::Low => RelevanceBand::Low,
        },
        confidence: ConfidenceLevel::High,
        priority: context_priority(&candidate.priority),
        freshness_state: candidate.freshness_state.clone(),
        user_selection: UserSelectionState::Auto,
        selected_at: request.created_at.clone(),
    }
}

fn task_type_value(task_type: &ContextTaskType) -> &'static str {
    match task_type {
        ContextTaskType::SourceBreakdown => "source-breakdown",
        ContextTaskType::CreativeDiscussion => "creative-discussion",
        ContextTaskType::OutlineGeneration => "outline-generation",
        ContextTaskType::CharacterSimulation => "character-simulation",
        ContextTaskType::ScenePlanning => "scene-planning",
        ContextTaskType::DraftGeneration => "draft-generation",
        ContextTaskType::StyleRevision => "style-revision",
        ContextTaskType::ContinuityAudit => "continuity-audit",
        ContextTaskType::WritebackSuggestion => "writeback-suggestion",
    }
}

fn activation_reason(
    request: &BuildContextPackRequest,
    candidate: &CandidateObject,
) -> ActivationReason {
    if candidate.source_ref == request.target_ref {
        ActivationReason::TargetMatch
    } else if candidate.required {
        ActivationReason::PinnedRequired
    } else {
        ActivationReason::TaskDefault
    }
}

fn context_priority(priority: &CandidatePriority) -> ContextPriority {
    match priority {
        CandidatePriority::Required => ContextPriority::Required,
        CandidatePriority::High => ContextPriority::High,
        CandidatePriority::Normal => ContextPriority::Normal,
        CandidatePriority::Low => ContextPriority::Low,
    }
}

fn included_item(candidate: &CandidateObject) -> ContextIncludedItem {
    ContextIncludedItem {
        source_ref: candidate.source_ref.clone(),
        source_object_type: candidate.source_object_type.clone(),
        source_schema_version: candidate.source_schema_version,
        content_hash: candidate.content_hash.clone(),
        activation_ref: activation_id(&candidate.source_ref),
    }
}

fn exclusion(
    request: &BuildContextPackRequest,
    candidate: &CandidateObject,
    reason: ContextExclusionReason,
    risk_kind: ContextExclusionRiskKind,
    explanation: &str,
) -> ContextExclusion {
    ContextExclusion {
        id: format!("exclusion-{}", candidate.source_ref),
        context_pack_id: request.pack_id.clone(),
        excluded_ref: candidate.source_ref.clone(),
        source_object_type: candidate.source_object_type.clone(),
        reason,
        risk_kind,
        explanation: explanation.to_string(),
        candidate_score: Some(match candidate.priority {
            CandidatePriority::Required | CandidatePriority::High => RelevanceBand::High,
            CandidatePriority::Normal => RelevanceBand::Medium,
            CandidatePriority::Low => RelevanceBand::Low,
        }),
        would_fit_if_budget: None,
        related_activation_ref: None,
        created_at: request.created_at.clone(),
    }
}

fn reject_or_exclude(
    request: &BuildContextPackRequest,
    candidate: &CandidateObject,
    reason: ContextExclusionReason,
    risk_kind: ContextExclusionRiskKind,
    explanation: &str,
    exclusions: &mut Vec<ContextExclusion>,
) -> Result<(), BuildContextPackError> {
    if candidate.required {
        return Err(BuildContextPackError::RequiredCandidateExcluded {
            source_ref: candidate.source_ref.clone(),
            reason,
        });
    }
    exclusions.push(exclusion(
        request,
        candidate,
        reason,
        risk_kind,
        explanation,
    ));
    Ok(())
}

fn context_budget(request: &BuildContextPackRequest) -> ContextBudget {
    ContextBudget {
        id: format!("budget-{}", request.pack_id),
        context_pack_id: request.pack_id.clone(),
        token_budget: TokenBudget {
            max_input_tokens: 1200,
            reserved_output_tokens: 600,
            estimation_method: "deterministic-toy-estimate".to_string(),
        },
        fragment_budget: FragmentBudget {
            max_fragments: 12,
            max_fragment_chars: 800,
        },
        object_budget: request
            .budget
            .object_limits
            .iter()
            .map(|limit| ObjectBudget {
                object_type: limit.object_type.clone(),
                max_objects: limit.max_objects as u32,
            })
            .collect(),
        priority_bands: PriorityBands {
            required: 1,
            high: 3,
            normal: 2,
            low: 0,
        },
        trimming_strategy: TrimmingStrategy::DropLowPriority,
        excluded_object_types: Vec::new(),
        excluded_refs: Vec::new(),
        overflow_policy: OverflowPolicy::FailClosed,
    }
}

fn context_hash(
    request: &BuildContextPackRequest,
    included: &[CandidateObject],
    exclusions: &[ContextExclusion],
    activations: &[ContextActivation],
) -> ContextHash {
    let included_hash_inputs: Vec<_> = included
        .iter()
        .map(|candidate| hash_input_entry(candidate, &candidate.content_hash))
        .collect();
    let excluded_hash_inputs: Vec<_> = exclusions
        .iter()
        .map(|exclusion| HashInputManifestEntry {
            source_object_type: exclusion.source_object_type.clone(),
            source_ref: exclusion.excluded_ref.clone(),
            source_schema_version: 1,
            hash: exclusion_hash(exclusion),
        })
        .collect();
    let activation_hash_inputs: Vec<_> = activations
        .iter()
        .map(|activation| HashInputManifestEntry {
            source_object_type: activation.source_object_type.clone(),
            source_ref: activation.source_ref.clone(),
            source_schema_version: activation.source_schema_version,
            hash: activation_hash(activation),
        })
        .collect();
    let source_hash_inputs: Vec<_> = request
        .source_hash_refs
        .iter()
        .map(|(source_ref, hash)| HashInputManifestEntry {
            source_object_type: "Source".to_string(),
            source_ref: source_ref.clone(),
            source_schema_version: 1,
            hash: hash.clone(),
        })
        .collect();

    let input = ContextHashInput {
        schema_version_map: vec![SchemaVersion {
            schema_id: "https://infinite-typewriter.local/schemas/context-pack.schema.json"
                .to_string(),
            version: 1,
        }],
        library_manifest_hash: request.library_manifest_hash.clone(),
        source_hash_refs: source_hash_inputs,
        change_set_refs: request.change_set_refs.clone(),
        change_set_hash: request.change_set_hash.clone(),
        selection_policy_version: request.selection_policy_version.clone(),
        included_item_hashes: included_hash_inputs,
        activation_hashes: activation_hash_inputs,
        excluded_item_hashes: excluded_hash_inputs,
    };
    let value = canonical_context_hash(&input);

    ContextHash {
        algorithm: HashAlgorithm::Sha256,
        canonicalization: Canonicalization::JsonCanonicalV1,
        schema_version_map: vec![SchemaVersionEntry {
            schema_id: "https://infinite-typewriter.local/schemas/context-pack.schema.json"
                .to_string(),
            version: 1,
        }],
        library_manifest_hash: request.library_manifest_hash.clone(),
        source_hash_refs: request
            .source_hash_refs
            .iter()
            .map(|(r#ref, hash)| ManifestHashEntry {
                r#ref: r#ref.clone(),
                hash: hash.clone(),
            })
            .collect(),
        change_set_refs: request.change_set_refs.clone(),
        change_set_hash: request.change_set_hash.clone(),
        selection_policy_version: request.selection_policy_version.clone(),
        prompt_template_version: None,
        included_item_hashes: included
            .iter()
            .map(|candidate| ManifestHashEntry {
                r#ref: candidate.source_ref.clone(),
                hash: candidate.content_hash.clone(),
            })
            .collect(),
        activation_hashes: activations
            .iter()
            .map(|activation| ManifestHashEntry {
                r#ref: activation.source_ref.clone(),
                hash: activation_hash(activation),
            })
            .collect(),
        excluded_item_hashes: Some(
            exclusions
                .iter()
                .map(|exclusion| ManifestHashEntry {
                    r#ref: exclusion.excluded_ref.clone(),
                    hash: exclusion_hash(exclusion),
                })
                .collect(),
        ),
        value,
    }
}

fn hash_input_entry(candidate: &CandidateObject, hash: &str) -> HashInputManifestEntry {
    HashInputManifestEntry {
        source_object_type: candidate.source_object_type.clone(),
        source_ref: candidate.source_ref.clone(),
        source_schema_version: candidate.source_schema_version,
        hash: hash.to_string(),
    }
}

fn exclusion_hash(exclusion: &ContextExclusion) -> String {
    let value = serde_json::to_value(exclusion).expect("exclusion must serialize for hash");
    let bytes = serde_json::to_vec(&value).expect("exclusion hash input must serialize");
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn activation_hash(activation: &ContextActivation) -> String {
    let value = serde_json::to_value(activation).expect("activation must serialize for hash");
    let bytes = serde_json::to_vec(&value).expect("activation hash input must serialize");
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn activation_id(source_ref: &str) -> StableId {
    format!("activation-{source_ref}")
}
