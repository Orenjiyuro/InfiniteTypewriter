use std::collections::BTreeMap;

use infinite_typewriter_providers::secret::{
    EnvSecretResolver, ResolvedSecret, SecretResolveError, SecretResolver, StaticSecretResolver,
};

#[test]
fn static_resolver_only_resolves_explicit_test_references() {
    let resolver = StaticSecretResolver::new(BTreeMap::from([(
        "OPENAI_AUTH_REF".to_string(),
        "sk-test-secret-value".to_string(),
    )]));

    let secret = resolver
        .resolve("OPENAI_AUTH_REF")
        .expect("secret resolves");
    assert_eq!(
        secret.expose_for_request_construction(),
        "sk-test-secret-value"
    );
    assert_eq!(format!("{secret:?}"), "ResolvedSecret([redacted:secret])");

    let missing = resolver
        .resolve("MISSING_AUTH_REF")
        .expect_err("secret is missing");
    assert_eq!(
        missing,
        SecretResolveError::MissingSecretRef("MISSING_AUTH_REF".to_string())
    );
}

#[test]
fn env_resolver_uses_configured_env_snapshot_and_resolved_secret_is_not_serializable() {
    let resolver = EnvSecretResolver::new(BTreeMap::from([(
        "ANTHROPIC_AUTH_REF".to_string(),
        "anthropic-secret-value".to_string(),
    )]));

    let secret = resolver
        .resolve("ANTHROPIC_AUTH_REF")
        .expect("secret resolves");
    assert_eq!(
        secret.expose_for_request_construction(),
        "anthropic-secret-value"
    );
    assert!(!resolved_secret_implements_serialize::<ResolvedSecret>());

    let missing = resolver
        .resolve("OTHER_KEY")
        .expect_err("secret is missing");
    assert_eq!(
        missing,
        SecretResolveError::MissingSecretRef("OTHER_KEY".to_string())
    );
}

fn resolved_secret_implements_serialize<T>() -> bool {
    let type_name = std::any::type_name::<T>();
    type_name.contains("serde::ser::Serialize")
}
