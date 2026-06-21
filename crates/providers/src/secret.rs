use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ResolvedSecret {
    value: String,
}

impl ResolvedSecret {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn expose_for_request_construction(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for ResolvedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResolvedSecret([redacted:secret])")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretResolveError {
    MissingSecretRef(String),
}

pub trait SecretResolver {
    fn resolve(&self, secret_ref: &str) -> Result<ResolvedSecret, SecretResolveError>;
}

#[derive(Debug, Clone, Default)]
pub struct StaticSecretResolver {
    secrets: BTreeMap<String, String>,
}

impl StaticSecretResolver {
    pub fn new(secrets: BTreeMap<String, String>) -> Self {
        Self { secrets }
    }
}

impl SecretResolver for StaticSecretResolver {
    fn resolve(&self, secret_ref: &str) -> Result<ResolvedSecret, SecretResolveError> {
        self.secrets
            .get(secret_ref)
            .cloned()
            .map(ResolvedSecret::new)
            .ok_or_else(|| SecretResolveError::MissingSecretRef(secret_ref.to_string()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnvSecretResolver {
    env_snapshot: BTreeMap<String, String>,
}

impl EnvSecretResolver {
    pub fn new(env_snapshot: BTreeMap<String, String>) -> Self {
        Self { env_snapshot }
    }
}

impl SecretResolver for EnvSecretResolver {
    fn resolve(&self, secret_ref: &str) -> Result<ResolvedSecret, SecretResolveError> {
        self.env_snapshot
            .get(secret_ref)
            .cloned()
            .map(ResolvedSecret::new)
            .ok_or_else(|| SecretResolveError::MissingSecretRef(secret_ref.to_string()))
    }
}
