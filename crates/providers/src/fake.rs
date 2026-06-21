use crate::model::{
    ProviderActor, ProviderEventSeverity, ProviderJob, ProviderJobEvent, ProviderJobEventType,
    ProviderJobStatus, ProviderKind, ProviderUsage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FakeProviderScenario {
    Success,
    ProviderError,
    Timeout,
    Cancelled,
    AwaitingApproval,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FakeProviderResult {
    pub final_status: ProviderJobStatus,
    pub events: Vec<ProviderJobEvent>,
    pub usage: Option<ProviderUsage>,
    pub draft_output_ref: Option<String>,
}

pub fn run_fake_provider(job: &ProviderJob, scenario: FakeProviderScenario) -> FakeProviderResult {
    match scenario {
        FakeProviderScenario::Success => success(job),
        FakeProviderScenario::ProviderError => terminal_with_event(
            job,
            ProviderJobStatus::Failed,
            ProviderJobEventType::ErrorRecorded,
            "Fake provider error recorded.",
        ),
        FakeProviderScenario::Timeout => terminal_with_event(
            job,
            ProviderJobStatus::Failed,
            ProviderJobEventType::Timeout,
            "Fake provider timeout recorded.",
        ),
        FakeProviderScenario::Cancelled => FakeProviderResult {
            final_status: ProviderJobStatus::Cancelled,
            events: vec![
                event(
                    job,
                    0,
                    ProviderJobEventType::CancelRequested,
                    ProviderJobStatus::Running,
                    ProviderJobStatus::Cancelled,
                    "Fake cancellation requested.",
                ),
                event(
                    job,
                    1,
                    ProviderJobEventType::Cancelled,
                    ProviderJobStatus::Running,
                    ProviderJobStatus::Cancelled,
                    "Fake provider job cancelled.",
                ),
            ],
            usage: None,
            draft_output_ref: None,
        },
        FakeProviderScenario::AwaitingApproval => FakeProviderResult {
            final_status: ProviderJobStatus::AwaitingApproval,
            events: vec![event(
                job,
                0,
                ProviderJobEventType::ApprovalRequested,
                ProviderJobStatus::Running,
                ProviderJobStatus::AwaitingApproval,
                "Fake provider requested visible user approval.",
            )],
            usage: None,
            draft_output_ref: None,
        },
    }
}

fn success(job: &ProviderJob) -> FakeProviderResult {
    let draft_output_ref = format!("draft-output-{}", job.id);
    FakeProviderResult {
        final_status: ProviderJobStatus::Succeeded,
        events: vec![
            event(
                job,
                0,
                ProviderJobEventType::ProviderStarted,
                ProviderJobStatus::ReadyForProvider,
                ProviderJobStatus::Running,
                "Fake provider started.",
            ),
            event(
                job,
                1,
                ProviderJobEventType::ProviderFinished,
                ProviderJobStatus::Running,
                ProviderJobStatus::Succeeded,
                "Fake provider finished.",
            ),
            event(
                job,
                2,
                ProviderJobEventType::DraftCreated,
                ProviderJobStatus::Running,
                ProviderJobStatus::Succeeded,
                "Fake draft output reference created.",
            ),
            event(
                job,
                3,
                ProviderJobEventType::UsageRecorded,
                ProviderJobStatus::Running,
                ProviderJobStatus::Succeeded,
                "Fake non-billable usage recorded.",
            ),
        ],
        usage: Some(ProviderUsage {
            provider_kind: ProviderKind::FakeTest,
            model: job.model.clone(),
            input_tokens: Some(12),
            output_tokens: Some(8),
            cached_input_tokens: None,
            reasoning_tokens: None,
            total_tokens: Some(20),
            provider_usage_raw_summary: Some(format!(
                "fake:{}:{}:{}",
                job.id, job.context_hash.value, job.prompt_template_version
            )),
            local_prompt_eval_count: None,
            local_eval_count: None,
            duration_ms: 1,
            request_id: Some(format!("fake-request-{}", job.id)),
            usage_unavailable_reason: None,
            usage_recorded_at: job
                .started_at
                .clone()
                .unwrap_or_else(|| job.created_at.clone()),
            is_billable: false,
        }),
        draft_output_ref: Some(draft_output_ref),
    }
}

fn terminal_with_event(
    job: &ProviderJob,
    final_status: ProviderJobStatus,
    event_type: ProviderJobEventType,
    summary: &str,
) -> FakeProviderResult {
    FakeProviderResult {
        final_status: final_status.clone(),
        events: vec![event(
            job,
            0,
            event_type,
            ProviderJobStatus::Running,
            final_status,
            summary,
        )],
        usage: None,
        draft_output_ref: None,
    }
}

fn event(
    job: &ProviderJob,
    sequence: u32,
    event_type: ProviderJobEventType,
    status_before: ProviderJobStatus,
    status_after: ProviderJobStatus,
    summary: &str,
) -> ProviderJobEvent {
    ProviderJobEvent {
        id: format!("event-{}-{}", job.id, sequence),
        job_id: job.id.clone(),
        sequence,
        event_type,
        at: job
            .started_at
            .clone()
            .unwrap_or_else(|| job.created_at.clone()),
        actor: ProviderActor::Provider,
        status_before: Some(status_before),
        status_after: Some(status_after),
        summary: summary.to_string(),
        redacted_payload_ref: None,
        raw_payload_retained: false,
        severity: ProviderEventSeverity::Info,
        error_class: None,
    }
}
