use std::collections::BTreeMap;

use infinite_typewriter_core::{AuditTrailEntry, ContextHash, ContextTaskType, StableId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type IsoDateTime = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderKind {
    ApiOpenaiCompatible,
    ApiAnthropic,
    ApiGemini,
    ApiOllama,
    CliCodex,
    CliClaudeCode,
    FakeTest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderAuthMode {
    None,
    EnvVar,
    KeychainRef,
    ManualSession,
    LocalService,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderCapabilityKind {
    TextGeneration,
    StructuredOutput,
    Streaming,
    ToolUse,
    LocalCliAgent,
    LocalModel,
    UsageReporting,
    Cancellation,
    PermissionGate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderSupportLevel {
    Native,
    Emulated,
    Unverified,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderJobStatus {
    Queued,
    BuildingContext,
    ReadyForProvider,
    Running,
    AwaitingApproval,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderJobEventType {
    JobCreated,
    ContextAttached,
    PromptBuilt,
    ProviderRequestBuilt,
    CliCommandBuilt,
    ProviderStarted,
    StdoutChunk,
    StderrChunk,
    StreamChunk,
    ApprovalRequested,
    ApprovalResolved,
    ProviderFinished,
    DraftCreated,
    UsageRecorded,
    ErrorRecorded,
    CancelRequested,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderActor {
    User,
    System,
    Provider,
    CliProcess,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderEventSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderErrorClass {
    AuthError,
    PermissionError,
    RateLimit,
    QuotaOrBilling,
    RequestInvalid,
    ContextTooLarge,
    BudgetBlocked,
    ProviderTimeout,
    ProviderOverloaded,
    ProviderInternal,
    NetworkError,
    SchemaOutputInvalid,
    SafetyBlocked,
    LocalServiceUnavailable,
    ModelNotFound,
    SecretUnresolved,
    AdvancedAgentModeDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderOutputMode {
    StructuredDraft,
    FreeformDraft,
    NonWritebackSummary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    pub schema_version: u16,
    pub id: StableId,
    #[serde(rename = "type")]
    pub object_type: String,
    pub label: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub status: String,
    pub tags: Vec<String>,
    pub metadata: BTreeMap<String, Value>,
    pub audit_trail: Vec<AuditTrailEntry>,
    pub provider_kind: ProviderKind,
    pub enabled: bool,
    pub auth_mode: ProviderAuthMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub default_model: String,
    pub default_timeout_ms: u64,
    pub capability_refs: Vec<StableId>,
    pub redaction_policy_version: String,
    pub advanced_agent_mode_enabled: bool,
    pub fail_closed_defaults: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapability {
    pub id: StableId,
    pub provider_config_id: StableId,
    pub capability_kind: ProviderCapabilityKind,
    pub supported: bool,
    pub support_level: ProviderSupportLevel,
    pub notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<IsoDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPermissionSet {
    pub allow_long_term_write: bool,
    pub allow_filesystem_write: bool,
    pub allow_shell: bool,
    pub allow_network: bool,
    pub allow_tools: bool,
    pub requires_user_approval: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_permission_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsage {
    pub provider_kind: ProviderKind,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_usage_raw_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_prompt_eval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_eval_count: Option<u64>,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_unavailable_reason: Option<String>,
    pub usage_recorded_at: IsoDateTime,
    pub is_billable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderError {
    pub error_class: ProviderErrorClass,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_status_code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_request_id: Option<String>,
    pub retryable: bool,
    pub redaction_policy_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCancellation {
    pub requested: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<IsoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<ProviderActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub graceful_termination_attempted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_signal: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderJobEvent {
    pub id: StableId,
    pub job_id: StableId,
    pub sequence: u32,
    pub event_type: ProviderJobEventType,
    pub at: IsoDateTime,
    pub actor: ProviderActor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_before: Option<ProviderJobStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_after: Option<ProviderJobStatus>,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_payload_ref: Option<StableId>,
    pub raw_payload_retained: bool,
    pub severity: ProviderEventSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_class: Option<ProviderErrorClass>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderJob {
    pub schema_version: u16,
    pub id: StableId,
    #[serde(rename = "type")]
    pub object_type: String,
    pub label: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub status: ProviderJobStatus,
    pub tags: Vec<String>,
    pub metadata: BTreeMap<String, Value>,
    pub audit_trail: Vec<AuditTrailEntry>,
    pub task_type: ContextTaskType,
    pub target_ref: StableId,
    pub context_pack_ref: StableId,
    pub context_hash: ContextHash,
    pub library_manifest_hash: String,
    pub provider_config_ref: StableId,
    pub provider_kind: ProviderKind,
    pub model: String,
    pub prompt_template_version: String,
    pub output_schema_ref: String,
    pub output_schema_version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_mode: Option<ProviderOutputMode>,
    pub permissions: ProviderPermissionSet,
    pub created_by: ProviderActor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<IsoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<IsoDateTime>,
    pub timeout_ms: u64,
    pub cancellation: ProviderCancellation,
    pub input_summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_output_ref: Option<StableId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ProviderUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProviderError>,
    pub event_refs: Vec<StableId>,
}
