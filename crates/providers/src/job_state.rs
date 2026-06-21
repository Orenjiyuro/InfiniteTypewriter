use std::fmt;

use crate::model::ProviderJobStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStateError {
    InvalidTransition {
        from: ProviderJobStatus,
        to: ProviderJobStatus,
        code: &'static str,
    },
}

impl fmt::Display for JobStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStateError::InvalidTransition { from, to, code } => {
                write!(f, "{code}: cannot transition from {from:?} to {to:?}")
            }
        }
    }
}

impl std::error::Error for JobStateError {}

pub fn validate_transition(
    from: &ProviderJobStatus,
    to: &ProviderJobStatus,
) -> Result<(), JobStateError> {
    if is_allowed_transition(from, to) {
        Ok(())
    } else {
        Err(JobStateError::InvalidTransition {
            from: from.clone(),
            to: to.clone(),
            code: "provider-job-invalid-transition",
        })
    }
}

pub fn can_provider_runner_execute(status: &ProviderJobStatus) -> bool {
    matches!(status, ProviderJobStatus::ReadyForProvider)
}

fn is_allowed_transition(from: &ProviderJobStatus, to: &ProviderJobStatus) -> bool {
    matches!(
        (from, to),
        (
            ProviderJobStatus::Queued,
            ProviderJobStatus::BuildingContext
        ) | (
            ProviderJobStatus::BuildingContext,
            ProviderJobStatus::ReadyForProvider
        ) | (
            ProviderJobStatus::ReadyForProvider,
            ProviderJobStatus::Running
        ) | (
            ProviderJobStatus::Running,
            ProviderJobStatus::AwaitingApproval
        ) | (
            ProviderJobStatus::AwaitingApproval,
            ProviderJobStatus::Running
        ) | (ProviderJobStatus::Running, ProviderJobStatus::Succeeded)
            | (ProviderJobStatus::Running, ProviderJobStatus::Failed)
            | (
                ProviderJobStatus::ReadyForProvider,
                ProviderJobStatus::Cancelled
            )
            | (ProviderJobStatus::Running, ProviderJobStatus::Cancelled)
            | (
                ProviderJobStatus::AwaitingApproval,
                ProviderJobStatus::Cancelled
            )
    )
}
