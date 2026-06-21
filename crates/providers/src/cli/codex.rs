use crate::cli::{CliCommandBuildInput, CliCommandSummary};
use crate::redaction::redact_env;

pub fn build_codex_command(input: CliCommandBuildInput) -> CliCommandSummary {
    let mut args = vec![
        "exec".to_string(),
        "-m".to_string(),
        input.model,
        "-C".to_string(),
        input.cwd.clone(),
        "--sandbox".to_string(),
        if input.advanced_agent_mode_enabled {
            "workspace-write".to_string()
        } else {
            "read-only".to_string()
        },
        "--ephemeral".to_string(),
        "--json".to_string(),
    ];

    if let Some(output_schema_path) = input.output_schema_path {
        args.push("--output-schema".to_string());
        args.push(output_schema_path);
    }
    args.push(input.prompt);

    CliCommandSummary {
        executable: "codex".to_string(),
        args,
        cwd: input.cwd,
        timeout_ms: input.timeout_ms,
        env_summary: redact_env(&input.env).value,
        non_interactive: true,
        permission_mode: if input.advanced_agent_mode_enabled {
            "workspace-write"
        } else {
            "read-only"
        }
        .to_string(),
        capture_stdout: true,
        capture_stderr: true,
        advanced_agent_mode_enabled: input.advanced_agent_mode_enabled,
    }
}
