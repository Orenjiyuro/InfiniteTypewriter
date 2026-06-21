use crate::job_state::can_provider_runner_execute;
use crate::model::{ProviderAuthMode, ProviderConfig, ProviderJob};

#[derive(Debug, Clone)]
pub struct RunnerPreflightInput<'a> {
    pub job: &'a ProviderJob,
    pub provider_config: Option<&'a ProviderConfig>,
    pub secret_resolved: bool,
    pub output_schema_available: bool,
    pub required_context_budget_blocked: bool,
    pub advanced_agent_mode_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunnerPreflightError {
    MissingProviderConfig,
    ProviderDisabled,
    JobNotReadyForProvider,
    SecretUnresolved,
    MissingContextHash,
    MissingPromptTemplateVersion,
    MissingOutputSchema,
    RequiredContextBudgetBlocked,
    AdvancedAgentModeDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerDraftResult {
    pub draft_output_ref: String,
    pub long_term_write_event_emitted: bool,
}

pub fn validate_runner_preflight(
    input: RunnerPreflightInput<'_>,
) -> Result<(), RunnerPreflightError> {
    let config = input
        .provider_config
        .ok_or(RunnerPreflightError::MissingProviderConfig)?;

    if !config.enabled {
        return Err(RunnerPreflightError::ProviderDisabled);
    }
    if !can_provider_runner_execute(&input.job.status) {
        return Err(RunnerPreflightError::JobNotReadyForProvider);
    }
    if requires_secret(&config.auth_mode) && !input.secret_resolved {
        return Err(RunnerPreflightError::SecretUnresolved);
    }
    if input.job.context_hash.value.is_empty() {
        return Err(RunnerPreflightError::MissingContextHash);
    }
    if input.job.prompt_template_version.is_empty() {
        return Err(RunnerPreflightError::MissingPromptTemplateVersion);
    }
    if input.job.output_schema_ref.is_empty() || !input.output_schema_available {
        return Err(RunnerPreflightError::MissingOutputSchema);
    }
    if input.required_context_budget_blocked {
        return Err(RunnerPreflightError::RequiredContextBudgetBlocked);
    }
    if input.advanced_agent_mode_requested && !config.advanced_agent_mode_enabled {
        return Err(RunnerPreflightError::AdvancedAgentModeDisabled);
    }

    Ok(())
}

pub fn successful_draft_result(_job: &ProviderJob, draft_output_ref: &str) -> RunnerDraftResult {
    RunnerDraftResult {
        draft_output_ref: draft_output_ref.to_string(),
        long_term_write_event_emitted: false,
    }
}

fn requires_secret(auth_mode: &ProviderAuthMode) -> bool {
    matches!(
        auth_mode,
        ProviderAuthMode::EnvVar | ProviderAuthMode::KeychainRef | ProviderAuthMode::ManualSession
    )
}
