use infinite_typewriter_core::context_pack::hash::{
    canonical_context_hash, ContextHashInput, HashManifestEntry, SchemaVersion,
};
use infinite_typewriter_core::context_pack::{
    build_context_pack, BudgetRequest, BuildContextPackError, BuildContextPackRequest,
    CandidateObject, CandidatePriority, ConfirmedChangeSetImpact, ObjectBudgetLimit,
};
use infinite_typewriter_core::{
    BreakdownRecipe, ContextExclusionReason, ContextTaskType, FreshnessState, SceneCard,
};

#[test]
fn canonical_hash_is_stable_for_manifest_order() {
    let mut first = base_hash_input();
    first.included_item_hashes = vec![
        manifest_entry(
            "Beat",
            "beat-toy",
            1,
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ),
        manifest_entry(
            "SceneCard",
            "scene-card-toy",
            1,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
    ];
    first.excluded_item_hashes = vec![
        manifest_entry(
            "StyleProfile",
            "style-profile-extra-toy",
            1,
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        ),
        manifest_entry(
            "CharacterDynamicState",
            "character-state-stale-toy",
            1,
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        ),
    ];

    let mut second = base_hash_input();
    second.included_item_hashes = vec![
        manifest_entry(
            "SceneCard",
            "scene-card-toy",
            1,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        manifest_entry(
            "Beat",
            "beat-toy",
            1,
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ),
    ];
    second.excluded_item_hashes = vec![
        manifest_entry(
            "CharacterDynamicState",
            "character-state-stale-toy",
            1,
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        ),
        manifest_entry(
            "StyleProfile",
            "style-profile-extra-toy",
            1,
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        ),
    ];

    assert_eq!(
        canonical_context_hash(&first),
        canonical_context_hash(&second)
    );
}

#[test]
fn canonical_hash_changes_when_audited_inputs_change() {
    let baseline = canonical_context_hash(&base_hash_input());

    let mut changed_schema = base_hash_input();
    changed_schema.schema_version_map[0].version = 2;
    assert_ne!(baseline, canonical_context_hash(&changed_schema));

    let mut changed_manifest = base_hash_input();
    changed_manifest.library_manifest_hash =
        "1111111111111111111111111111111111111111111111111111111111111111".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_manifest));

    let mut changed_source = base_hash_input();
    changed_source.source_hash_refs[0].hash =
        "2222222222222222222222222222222222222222222222222222222222222222".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_source));

    let mut changed_change_set = base_hash_input();
    changed_change_set.change_set_hash =
        "3333333333333333333333333333333333333333333333333333333333333333".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_change_set));

    let mut changed_item = base_hash_input();
    changed_item.included_item_hashes[0].hash =
        "4444444444444444444444444444444444444444444444444444444444444444".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_item));

    let mut changed_policy = base_hash_input();
    changed_policy.selection_policy_version = "context-pack-selection-v2".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_policy));

    let mut changed_activation = base_hash_input();
    changed_activation.activation_hashes[0].hash =
        "5555555555555555555555555555555555555555555555555555555555555555".to_string();
    assert_ne!(baseline, canonical_context_hash(&changed_activation));
}

#[test]
fn canonical_story_scene_card_and_breakdown_exports_are_available() {
    fn assert_type<T>() {}

    assert_type::<SceneCard>();
    assert_type::<BreakdownRecipe>();
}

#[test]
fn draft_generation_includes_task_specific_scene_context() {
    let pack = build_context_pack(draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        candidate("beat-toy", "Beat", CandidatePriority::High),
        candidate(
            "character-state-toy",
            "CharacterDynamicState",
            CandidatePriority::High,
        ),
        candidate(
            "style-profile-toy",
            "StyleProfile",
            CandidatePriority::Normal,
        ),
        irrelevant_candidate("continuity-issue-toy", "ContinuityIssue"),
    ]))
    .expect("draft context should build");

    let included_types: Vec<_> = pack
        .included_items
        .iter()
        .map(|item| item.source_object_type.as_str())
        .collect();
    assert_eq!(
        included_types,
        vec!["SceneCard", "Beat", "CharacterDynamicState", "StyleProfile"]
    );
    assert!(pack
        .activations
        .iter()
        .all(|activation| activation.confidence
            == infinite_typewriter_core::ContextConfidenceLevel::High));
    assert_eq!(
        pack.exclusions[0].reason,
        ContextExclusionReason::TaskIrrelevant
    );
}

#[test]
fn character_simulation_includes_profiles_state_relationship_and_outline_target() {
    let mut request = base_request(ContextTaskType::CharacterSimulation);
    request.target_ref = "simulation-context-toy".to_string();
    request.candidates = vec![
        required_candidate("simulation-context-toy", "SimulationContext"),
        candidate(
            "stable-profile-toy",
            "CharacterStableProfile",
            CandidatePriority::High,
        ),
        candidate(
            "dynamic-state-toy",
            "CharacterDynamicState",
            CandidatePriority::High,
        ),
        candidate(
            "relationship-temperature-toy",
            "RelationshipTemperature",
            CandidatePriority::High,
        ),
        candidate("chapter-plan-toy", "ChapterPlan", CandidatePriority::Normal),
    ];

    let pack = build_context_pack(request).expect("simulation context should build");
    let included_types: Vec<_> = pack
        .included_items
        .iter()
        .map(|item| item.source_object_type.as_str())
        .collect();

    assert_eq!(
        included_types,
        vec![
            "SimulationContext",
            "CharacterStableProfile",
            "CharacterDynamicState",
            "RelationshipTemperature",
            "ChapterPlan"
        ]
    );
}

#[test]
fn continuity_audit_can_include_stale_objects_but_draft_generation_filters_them() {
    let stale_timeline = stale_candidate("character-timeline-old", "CharacterTimeline");
    let audit_pack = build_context_pack(audit_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        stale_timeline.clone(),
    ]))
    .expect("continuity audit can include stale history");

    let stale_activation = audit_pack
        .activations
        .iter()
        .find(|activation| activation.source_ref == "character-timeline-old")
        .expect("stale history should be included for audit");
    assert_eq!(stale_activation.freshness_state, FreshnessState::Stale);

    let draft_pack = build_context_pack(draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        stale_timeline,
    ]))
    .expect("draft context should build without stale history");
    assert_eq!(
        draft_pack.exclusions[0].reason,
        ContextExclusionReason::StaleOrSuperseded
    );
}

#[test]
fn confirmed_change_set_impact_filters_superseded_candidates() {
    let mut request = draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        candidate(
            "character-state-old-toy",
            "CharacterDynamicState",
            CandidatePriority::High,
        ),
    ]);
    request.confirmed_change_set_impacts = vec![ConfirmedChangeSetImpact {
        source_ref: "character-state-old-toy".to_string(),
        superseded_by_ref: "character-state-current-toy".to_string(),
        change_set_ref: "change-set-toy".to_string(),
    }];

    let pack = build_context_pack(request).expect("superseded optional state should be excluded");

    let exclusion = pack
        .exclusions
        .iter()
        .find(|exclusion| exclusion.excluded_ref == "character-state-old-toy")
        .expect("old state should be excluded by confirmed change set impact");
    assert_eq!(exclusion.reason, ContextExclusionReason::StaleOrSuperseded);
}

#[test]
fn builder_records_required_exclusion_reasons() {
    let pack = build_context_pack(draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        stale_candidate("state-stale-toy", "CharacterDynamicState"),
        user_excluded_candidate("state-user-excluded-toy", "CharacterDynamicState"),
        irrelevant_candidate("relationship-temperature-toy", "RelationshipTemperature"),
        insufficient_evidence_candidate("style-low-evidence-toy", "StyleProfile"),
        source_boundary_candidate("source-boundary-toy", "EvidenceCard"),
    ]))
    .expect("context should build with explainable exclusions");

    let reasons: Vec<_> = pack
        .exclusions
        .iter()
        .map(|exclusion| exclusion.reason.clone())
        .collect();
    assert!(reasons.contains(&ContextExclusionReason::StaleOrSuperseded));
    assert!(reasons.contains(&ContextExclusionReason::UserExcluded));
    assert!(reasons.contains(&ContextExclusionReason::TaskIrrelevant));
    assert!(reasons.contains(&ContextExclusionReason::InsufficientEvidence));
    assert!(reasons.contains(&ContextExclusionReason::SourceBoundary));
}

#[test]
fn required_items_over_budget_fail_closed() {
    let mut request = draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        required_candidate("beat-required-toy", "Beat"),
    ]);
    request.budget.max_included_items = Some(1);

    let error = build_context_pack(request).expect_err("required overflow must fail closed");

    assert_eq!(
        error,
        BuildContextPackError::RequiredBudgetOverflow {
            source_ref: "beat-required-toy".to_string()
        }
    );
}

#[test]
fn required_items_filtered_by_stale_or_evidence_fail_closed() {
    let stale_error = build_context_pack(draft_request(vec![stale_required_candidate(
        "scene-card-stale-toy",
        "SceneCard",
    )]))
    .expect_err("required stale candidate must fail closed");
    assert_eq!(
        stale_error,
        BuildContextPackError::RequiredCandidateExcluded {
            source_ref: "scene-card-stale-toy".to_string(),
            reason: ContextExclusionReason::StaleOrSuperseded
        }
    );

    let evidence_error = build_context_pack(draft_request(vec![
        required_insufficient_evidence_candidate("scene-card-low-evidence-toy", "SceneCard"),
    ]))
    .expect_err("required low-evidence candidate must fail closed");
    assert_eq!(
        evidence_error,
        BuildContextPackError::RequiredCandidateExcluded {
            source_ref: "scene-card-low-evidence-toy".to_string(),
            reason: ContextExclusionReason::InsufficientEvidence
        }
    );
}

#[test]
fn optional_low_priority_items_are_dropped_first_with_budget_exclusions() {
    let mut request = draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        candidate("beat-high-toy", "Beat", CandidatePriority::High),
        candidate("style-low-toy", "StyleProfile", CandidatePriority::Low),
    ]);
    request.budget.max_included_items = Some(2);

    let pack = build_context_pack(request).expect("optional overflow should be explainable");

    assert_eq!(
        pack.included_items
            .iter()
            .map(|item| item.source_ref.as_str())
            .collect::<Vec<_>>(),
        vec!["scene-card-toy", "beat-high-toy"]
    );
    assert_eq!(pack.exclusions[0].excluded_ref, "style-low-toy");
    assert_eq!(
        pack.exclusions[0].reason,
        ContextExclusionReason::BudgetInsufficient
    );
}

#[test]
fn object_budget_limits_object_types_independently() {
    let mut request = draft_request(vec![
        required_candidate("scene-card-toy", "SceneCard"),
        candidate("beat-one-toy", "Beat", CandidatePriority::High),
        candidate("beat-two-toy", "Beat", CandidatePriority::High),
        candidate("style-profile-toy", "StyleProfile", CandidatePriority::High),
    ]);
    request.budget.object_limits = vec![ObjectBudgetLimit {
        object_type: "Beat".to_string(),
        max_objects: 1,
    }];

    let pack = build_context_pack(request).expect("object budget should apply by type");

    assert!(pack
        .included_items
        .iter()
        .any(|item| item.source_ref == "style-profile-toy"));
    assert_eq!(
        pack.exclusions
            .iter()
            .find(|exclusion| exclusion.excluded_ref == "beat-two-toy")
            .expect("second beat should be excluded")
            .reason,
        ContextExclusionReason::BudgetInsufficient
    );
}

#[test]
fn preview_builder_output_has_no_provider_job_fields() {
    let pack = build_context_pack(draft_request(vec![required_candidate(
        "scene-card-toy",
        "SceneCard",
    )]))
    .expect("preview context should build locally");
    let value = serde_json::to_value(pack).expect("pack should serialize");
    let field_names = collect_field_names(&value);

    assert!(!field_names.contains(&"createdForJobRef".to_string()));
    for forbidden in [
        "provider",
        "providerConfig",
        "model",
        "modelId",
        "apiKey",
        "api_key",
        "cliCommand",
        "promptTemplateBody",
    ] {
        assert!(
            !field_names.contains(&forbidden.to_string()),
            "{forbidden} must not be part of 9.7 preview output"
        );
    }
}

fn base_hash_input() -> ContextHashInput {
    ContextHashInput {
        schema_version_map: vec![SchemaVersion {
            schema_id: "https://infinite-typewriter.local/schemas/context-pack.schema.json"
                .to_string(),
            version: 1,
        }],
        library_manifest_hash: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
            .to_string(),
        source_hash_refs: vec![manifest_entry(
            "Source",
            "source-toy",
            1,
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        )],
        change_set_refs: vec!["change-set-toy".to_string()],
        change_set_hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            .to_string(),
        selection_policy_version: "context-pack-selection-v1".to_string(),
        included_item_hashes: vec![manifest_entry(
            "SceneCard",
            "scene-card-toy",
            1,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )],
        activation_hashes: vec![manifest_entry(
            "SceneCard",
            "scene-card-toy",
            1,
            "9999999999999999999999999999999999999999999999999999999999999999",
        )],
        excluded_item_hashes: vec![],
    }
}

fn manifest_entry(
    source_object_type: &str,
    source_ref: &str,
    source_schema_version: u16,
    hash: &str,
) -> HashManifestEntry {
    HashManifestEntry {
        source_object_type: source_object_type.to_string(),
        source_ref: source_ref.to_string(),
        source_schema_version,
        hash: hash.to_string(),
    }
}

fn base_request(task_type: ContextTaskType) -> BuildContextPackRequest {
    BuildContextPackRequest {
        pack_id: "context-pack-builder-toy".to_string(),
        label: "Toy Builder Context".to_string(),
        created_at: "2026-06-06T00:00:00.000Z".to_string(),
        task_type,
        target_ref: "scene-card-toy".to_string(),
        created_for_task_ref: "task-builder-preview-toy".to_string(),
        library_manifest_hash: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
            .to_string(),
        source_hash_refs: vec![(
            "source-toy".to_string(),
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(),
        )],
        change_set_refs: vec!["change-set-toy".to_string()],
        change_set_hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            .to_string(),
        selection_policy_version: "context-pack-selection-v1".to_string(),
        confirmed_change_set_impacts: vec![],
        budget: BudgetRequest {
            max_included_items: None,
            object_limits: vec![],
        },
        candidates: vec![],
    }
}

fn draft_request(candidates: Vec<CandidateObject>) -> BuildContextPackRequest {
    let mut request = base_request(ContextTaskType::DraftGeneration);
    request.candidates = candidates;
    request
}

fn audit_request(candidates: Vec<CandidateObject>) -> BuildContextPackRequest {
    let mut request = base_request(ContextTaskType::ContinuityAudit);
    request.candidates = candidates;
    request
}

fn required_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::Required);
    candidate.required = true;
    candidate
}

fn candidate(
    source_ref: &str,
    source_object_type: &str,
    priority: CandidatePriority,
) -> CandidateObject {
    CandidateObject {
        source_ref: source_ref.to_string(),
        source_object_type: source_object_type.to_string(),
        source_schema_version: 1,
        content_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_string(),
        applicable_task_types: vec![
            ContextTaskType::DraftGeneration,
            ContextTaskType::ContinuityAudit,
            ContextTaskType::CharacterSimulation,
        ],
        required: false,
        priority,
        freshness_state: FreshnessState::Current,
        has_sufficient_evidence: true,
        permission_allowed: true,
        source_boundary_allowed: true,
        user_excluded: false,
    }
}

fn stale_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::High);
    candidate.freshness_state = FreshnessState::Stale;
    candidate
}

fn stale_required_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = required_candidate(source_ref, source_object_type);
    candidate.freshness_state = FreshnessState::Stale;
    candidate
}

fn user_excluded_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::High);
    candidate.user_excluded = true;
    candidate
}

fn irrelevant_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::Normal);
    candidate.applicable_task_types = vec![ContextTaskType::OutlineGeneration];
    candidate
}

fn insufficient_evidence_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::Normal);
    candidate.has_sufficient_evidence = false;
    candidate
}

fn required_insufficient_evidence_candidate(
    source_ref: &str,
    source_object_type: &str,
) -> CandidateObject {
    let mut candidate = required_candidate(source_ref, source_object_type);
    candidate.has_sufficient_evidence = false;
    candidate
}

fn source_boundary_candidate(source_ref: &str, source_object_type: &str) -> CandidateObject {
    let mut candidate = candidate(source_ref, source_object_type, CandidatePriority::Normal);
    candidate.source_boundary_allowed = false;
    candidate
}

fn collect_field_names(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(items) => items
            .iter()
            .flat_map(collect_field_names)
            .collect::<Vec<_>>(),
        serde_json::Value::Object(object) => object
            .iter()
            .flat_map(|(key, nested)| {
                let mut keys = vec![key.clone()];
                keys.extend(collect_field_names(nested));
                keys
            })
            .collect(),
        _ => Vec::new(),
    }
}
