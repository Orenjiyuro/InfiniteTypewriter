use crate::cli::{CliCommandBuildInput, CliCommandSummary};
use crate::redaction::redact_env;

pub fn build_claude_code_command(input: CliCommandBuildInput) -> CliCommandSummary {
    let mut args = vec![
        "--print".to_string(),
        "--bare".to_string(),
        "--model".to_string(),
        input.model,
        "--permission-mode".to_string(),
        if input.advanced_agent_mode_enabled {
            "default".to_string()
        } else {
            "plan".to_string()
        },
        "--tools".to_string(),
        if input.advanced_agent_mode_enabled {
            "default".to_string()
        } else {
            "".to_string()
        },
        "--disallowedTools".to_string(),
        if input.advanced_agent_mode_enabled {
            "".to_string()
        } else {
            "Bash,Edit,Write".to_string()
        },
        "--output-format".to_string(),
        "json".to_string(),
        "--no-session-persistence".to_string(),
    ];

    if let Some(output_schema_json) = input.output_schema_json {
        args.push("--json-schema".to_string());
        args.push(output_schema_json);
    }
    args.push(input.prompt);

    CliCommandSummary {
        executable: "claude".to_string(),
        args,
        cwd: input.cwd,
        timeout_ms: input.timeout_ms,
        env_summary: redact_env(&input.env).value,
        non_interactive: true,
        permission_mode: if input.advanced_agent_mode_enabled {
            "default"
        } else {
            "plan"
        }
        .to_string(),
        capture_stdout: true,
        capture_stderr: true,
        advanced_agent_mode_enabled: input.advanced_agent_mode_enabled,
    }
}
