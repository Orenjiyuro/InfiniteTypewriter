use std::collections::BTreeMap;

pub mod claude_code;
pub mod codex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommandBuildInput {
    pub model: String,
    pub cwd: String,
    pub prompt: String,
    pub timeout_ms: u64,
    pub env: BTreeMap<String, String>,
    pub output_schema_path: Option<String>,
    pub output_schema_json: Option<String>,
    pub advanced_agent_mode_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommandSummary {
    pub executable: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub timeout_ms: u64,
    pub env_summary: BTreeMap<String, String>,
    pub non_interactive: bool,
    pub permission_mode: String,
    pub capture_stdout: bool,
    pub capture_stderr: bool,
    pub advanced_agent_mode_enabled: bool,
}
