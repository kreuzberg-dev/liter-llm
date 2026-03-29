use dashmap::DashMap;

use crate::config::VirtualKeyConfig;

/// Context injected into request extensions after successful auth.
#[derive(Debug, Clone)]
pub struct KeyContext {
    pub key_id: String,
    pub allowed_models: Option<Vec<String>>,
    pub is_master: bool,
}

impl KeyContext {
    /// Create a context representing the master key (unrestricted access).
    pub fn master() -> Self {
        Self {
            key_id: "master".into(),
            allowed_models: None,
            is_master: true,
        }
    }

    /// Create a context from a virtual key configuration.
    pub fn from_config(config: &VirtualKeyConfig) -> Self {
        let allowed_models = if config.models.is_empty() {
            None
        } else {
            Some(config.models.clone())
        };
        Self {
            key_id: config.key.clone(),
            allowed_models,
            is_master: false,
        }
    }

    /// Returns true if this key is allowed to access the given model.
    pub fn can_access_model(&self, model: &str) -> bool {
        match &self.allowed_models {
            None => true,
            Some(models) => models.iter().any(|m| m == model),
        }
    }
}

/// In-memory virtual key store backed by `DashMap` for concurrent access.
pub struct KeyStore {
    keys: DashMap<String, VirtualKeyConfig>,
    master_key: Option<String>,
}

impl KeyStore {
    /// Build a key store from the proxy configuration values.
    pub fn from_config(master_key: Option<String>, keys: &[VirtualKeyConfig]) -> Self {
        let map = DashMap::new();
        for k in keys {
            map.insert(k.key.clone(), k.clone());
        }
        Self { keys: map, master_key }
    }

    /// Check whether `token` matches the configured master key.
    pub fn is_master_key(&self, token: &str) -> bool {
        self.master_key.as_deref() == Some(token)
    }

    /// Look up a virtual key configuration by its token string.
    pub fn get(&self, token: &str) -> Option<VirtualKeyConfig> {
        self.keys.get(token).map(|r| r.value().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_key_config(key: &str, models: Vec<String>) -> VirtualKeyConfig {
        VirtualKeyConfig {
            key: key.to_string(),
            description: None,
            models,
            rpm: Some(60),
            tpm: Some(100_000),
            budget_limit: Some(50.0),
        }
    }

    // ── KeyStore tests ──────────────────────────────────────────────────

    #[test]
    fn master_key_match_returns_true() {
        let store = KeyStore::from_config(Some("sk-master".into()), &[]);
        assert!(store.is_master_key("sk-master"));
    }

    #[test]
    fn master_key_mismatch_returns_false() {
        let store = KeyStore::from_config(Some("sk-master".into()), &[]);
        assert!(!store.is_master_key("sk-wrong"));
    }

    #[test]
    fn no_master_key_always_returns_false() {
        let store = KeyStore::from_config(None, &[]);
        assert!(!store.is_master_key("sk-anything"));
    }

    #[test]
    fn get_existing_key_returns_config() {
        let cfg = sample_key_config("vk-team-a", vec!["gpt-4o".into()]);
        let store = KeyStore::from_config(None, std::slice::from_ref(&cfg));

        let result = store.get("vk-team-a");
        assert!(result.is_some());
        let found = result.unwrap();
        assert_eq!(found.key, "vk-team-a");
        assert_eq!(found.models, vec!["gpt-4o"]);
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let store = KeyStore::from_config(None, &[]);
        assert!(store.get("vk-missing").is_none());
    }

    // ── KeyContext tests ────────────────────────────────────────────────

    #[test]
    fn master_context_has_no_restrictions() {
        let ctx = KeyContext::master();
        assert!(ctx.is_master);
        assert!(ctx.allowed_models.is_none());
        assert!(ctx.can_access_model("any-model"));
    }

    #[test]
    fn context_with_allowed_models_permits_listed_model() {
        let cfg = sample_key_config("vk-1", vec!["gpt-4o".into(), "claude-sonnet".into()]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(!ctx.is_master);
        assert!(ctx.can_access_model("gpt-4o"));
        assert!(ctx.can_access_model("claude-sonnet"));
    }

    #[test]
    fn context_with_allowed_models_denies_unlisted_model() {
        let cfg = sample_key_config("vk-1", vec!["gpt-4o".into()]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(!ctx.can_access_model("claude-sonnet"));
    }

    #[test]
    fn context_with_empty_models_allows_all() {
        let cfg = sample_key_config("vk-1", vec![]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(ctx.allowed_models.is_none());
        assert!(ctx.can_access_model("any-model"));
    }
}
