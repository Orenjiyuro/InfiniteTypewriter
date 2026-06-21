use infinite_typewriter_providers::job_state::{
    can_provider_runner_execute, validate_transition, JobStateError,
};
use infinite_typewriter_providers::ProviderJobStatus;

#[test]
fn allows_documented_provider_job_status_transitions() {
    let allowed = [
        (
            ProviderJobStatus::Queued,
            ProviderJobStatus::BuildingContext,
        ),
        (
            ProviderJobStatus::BuildingContext,
            ProviderJobStatus::ReadyForProvider,
        ),
        (
            ProviderJobStatus::ReadyForProvider,
            ProviderJobStatus::Running,
        ),
        (
            ProviderJobStatus::Running,
            ProviderJobStatus::AwaitingApproval,
        ),
        (
            ProviderJobStatus::AwaitingApproval,
            ProviderJobStatus::Running,
        ),
        (ProviderJobStatus::Running, ProviderJobStatus::Succeeded),
        (ProviderJobStatus::Running, ProviderJobStatus::Failed),
        (
            ProviderJobStatus::ReadyForProvider,
            ProviderJobStatus::Cancelled,
        ),
        (ProviderJobStatus::Running, ProviderJobStatus::Cancelled),
        (
            ProviderJobStatus::AwaitingApproval,
            ProviderJobStatus::Cancelled,
        ),
    ];

    for (from, to) in allowed {
        validate_transition(&from, &to).expect("transition is allowed");
    }
}

#[test]
fn rejects_forbidden_provider_job_status_transitions_with_stable_error() {
    let forbidden = [
        (ProviderJobStatus::Queued, ProviderJobStatus::Running),
        (ProviderJobStatus::Succeeded, ProviderJobStatus::Running),
        (ProviderJobStatus::Failed, ProviderJobStatus::Succeeded),
        (ProviderJobStatus::Cancelled, ProviderJobStatus::Running),
    ];

    for (from, to) in forbidden {
        let error = validate_transition(&from, &to).expect_err("transition is forbidden");
        assert!(matches!(error, JobStateError::InvalidTransition { .. }));
        assert!(error
            .to_string()
            .contains("provider-job-invalid-transition"));
    }
}

#[test]
fn provider_runner_only_executes_ready_for_provider_jobs() {
    assert!(can_provider_runner_execute(
        &ProviderJobStatus::ReadyForProvider
    ));
    assert!(!can_provider_runner_execute(
        &ProviderJobStatus::BuildingContext
    ));
    assert!(!can_provider_runner_execute(&ProviderJobStatus::Queued));
    assert!(!can_provider_runner_execute(&ProviderJobStatus::Running));
}
