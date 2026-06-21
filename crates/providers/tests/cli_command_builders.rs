use std::collections::BTreeMap;

use infinite_typewriter_providers::cli::{
    claude_code::build_claude_code_command, codex::build_codex_command, CliCommandBuildInput,
};

#[test]
fn codex_builder_records_non_interactive_read_only_command_boundary() {
    let command = build_codex_command(input(false));

    assert_eq!(command.executable, "codex");
    assert_eq!(command.args[0], "exec");
    assert!(command.args.contains(&"--ephemeral".to_string()));
    assert!(command.args.contains(&"--json".to_string()));
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["--sandbox", "read-only"]));
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["-C", "C:\\toy-workspace"]));
    assert_eq!(command.cwd, "C:\\toy-workspace");
    assert!(command.non_interactive);
    assert!(command.capture_stdout);
    assert!(command.capture_stderr);
    assert_eq!(command.permission_mode, "read-only");
    assert_eq!(command.env_summary["OPENAI_AUTH_REF"], "[redacted:secret]");
}

#[test]
fn claude_code_builder_uses_print_plan_mode_and_denies_write_tools_by_default() {
    let command = build_claude_code_command(input(false));

    assert_eq!(command.executable, "claude");
    assert!(command.args.contains(&"--print".to_string()));
    assert!(command.args.contains(&"--bare".to_string()));
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["--permission-mode", "plan"]));
    assert!(command.args.windows(2).any(|pair| pair == ["--tools", ""]));
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["--json-schema", "{\"type\":\"object\"}"]));
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["--disallowedTools", "Bash,Edit,Write"]));
    assert_eq!(command.cwd, "C:\\toy-workspace");
    assert!(command.non_interactive);
    assert_eq!(command.permission_mode, "plan");
}

#[test]
fn advanced_agent_mode_is_explicit_in_command_summary() {
    let command = build_claude_code_command(input(true));

    assert!(command.advanced_agent_mode_enabled);
    assert!(command
        .args
        .windows(2)
        .any(|pair| pair == ["--tools", "default"]));
}

fn input(advanced_agent_mode_enabled: bool) -> CliCommandBuildInput {
    CliCommandBuildInput {
        model: "toy-model".to_string(),
        cwd: "C:\\toy-workspace".to_string(),
        prompt: "toy public prompt".to_string(),
        timeout_ms: 30000,
        env: BTreeMap::from([
            (
                "OPENAI_AUTH_REF".to_string(),
                "sk-test-secret-value".to_string(),
            ),
            ("SAFE_FLAG".to_string(), "true".to_string()),
        ]),
        output_schema_path: Some("schemas\\toy-output.schema.json".to_string()),
        output_schema_json: Some("{\"type\":\"object\"}".to_string()),
        advanced_agent_mode_enabled,
    }
}
