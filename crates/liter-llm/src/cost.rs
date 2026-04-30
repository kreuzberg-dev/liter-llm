//! Cost estimation for LLM API calls.
//!
//! Pricing data is embedded at compile time from `schemas/pricing.json` and
//! covers the most commonly used models across major providers.  Prices are
//! approximate and derived from the [litellm](https://github.com/BerriAI/litellm)
//! project (MIT License, Copyright 2023 Berri AI).
//!
//! # Example
//!
//! ```rust
//! use liter_llm::cost;
//!
//! // Returns None for unknown models.
//! assert!(cost::completion_cost("unknown-model", 100, 50).is_none());
//!
//! // Returns Some(cost_in_usd) for known models.
//! let cost = cost::completion_cost("gpt-4o", 1000, 500).unwrap();
//! assert!(cost > 0.0);
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

use serde::Deserialize;

// Embedded at compile time so the binary is self-contained with no runtime
// file-system dependency.
const PRICING_JSON: &str = include_str!("../schemas/pricing.json");

/// Lazy-initialised registry parsed from the embedded JSON.
/// Stores a `Result` so that parse failures surface at call time rather than
/// panicking the process (mirrors the pattern used in `provider/mod.rs`).
static PRICING: LazyLock<std::result::Result<PricingRegistry, String>> =
    LazyLock::new(|| serde_json::from_str(PRICING_JSON).map_err(|e| e.to_string()));

/// Access the pricing registry, returning `None` if the embedded JSON was invalid.
///
/// Invalid embedded JSON is a compile-time defect; callers treat it the same
/// as an unknown model (no pricing available).
fn pricing() -> Option<&'static PricingRegistry> {
    PRICING.as_ref().ok()
}

// ─── Registry ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PricingRegistry {
    models: HashMap<String, ModelPricing>,
}

/// Per-token pricing for a single model (USD per token).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ModelPricing {
    /// Cost in USD per input (prompt) token.
    pub input_cost_per_token: f64,
    /// Cost in USD per output (completion) token.  Zero for embedding models.
    pub output_cost_per_token: f64,
    /// Cost in USD per cached input token (cache hit / read). When the model
    /// supports prompt caching the provider serves cached tokens at this
    /// discounted rate; otherwise this is `None` and cached tokens are billed
    /// at `input_cost_per_token`.
    #[serde(default)]
    pub cache_read_input_token_cost: Option<f64>,
    /// Cost in USD per token written to the prompt cache (Anthropic-style
    /// cache-write surcharge). `None` when the provider does not separately
    /// charge for cache writes.
    #[serde(default)]
    pub cache_creation_input_token_cost: Option<f64>,
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Calculate the estimated cost of a completion given a model name and token
/// counts.
///
/// Returns `None` if the model is not present in the embedded pricing registry.
/// Returns `Some(cost_usd)` otherwise, where the value is in US dollars.
///
/// When an exact model name match is not found, progressively shorter prefixes
/// are tried by stripping from the last `-` or `.` separator.  For example,
/// `gpt-4-0613` will match `gpt-4` if no `gpt-4-0613` entry exists.
///
/// # Example
///
/// ```rust
/// use liter_llm::cost;
///
/// let usd = cost::completion_cost("gpt-4o", 1_000, 500).unwrap();
/// // 1000 * 0.0000025 + 500 * 0.00001 = 0.0025 + 0.005 = 0.0075
/// assert!((usd - 0.0075).abs() < 1e-9);
/// ```
#[must_use]
pub fn completion_cost(model: &str, prompt_tokens: u64, completion_tokens: u64) -> Option<f64> {
    completion_cost_with_cache(model, prompt_tokens, 0, completion_tokens)
}

/// Calculate the estimated cost of a completion, accounting for cached
/// (cache-hit) prompt tokens billed at the provider's discounted rate.
///
/// `cached_tokens` is the count of prompt tokens served from the provider's
/// prompt cache. It must be `<= prompt_tokens` (cached tokens are a subset of
/// the prompt). The non-cached portion is billed at `input_cost_per_token`
/// and the cached portion at `cache_read_input_token_cost` when the model
/// has cache pricing; otherwise the entire prompt is billed at the regular
/// input rate.
///
/// Returns `None` if the model is not present in the embedded pricing
/// registry, mirroring [`completion_cost`].
#[must_use]
pub fn completion_cost_with_cache(
    model: &str,
    prompt_tokens: u64,
    cached_tokens: u64,
    completion_tokens: u64,
) -> Option<f64> {
    let pricing = model_pricing(model)?;
    let cached = cached_tokens.min(prompt_tokens);
    let uncached = prompt_tokens - cached;
    let cache_rate = pricing
        .cache_read_input_token_cost
        .unwrap_or(pricing.input_cost_per_token);
    Some(
        (uncached as f64) * pricing.input_cost_per_token
            + (cached as f64) * cache_rate
            + (completion_tokens as f64) * pricing.output_cost_per_token,
    )
}

/// Look up the per-token pricing for a model.
///
/// Returns `None` if the model is not present in the embedded pricing registry.
/// The returned reference is valid for the lifetime of the process (`'static`).
///
/// When an exact model name match is not found, progressively shorter prefixes
/// are tried by stripping from the last `-` or `.` separator.  For example,
/// `gpt-4-0613` will try `gpt-4-0613`, then `gpt-4`, then `gpt`.  The first
/// match wins.
#[must_use]
pub fn model_pricing(model: &str) -> Option<&'static ModelPricing> {
    let models = &pricing()?.models;

    // Exact match first.
    if let Some(p) = models.get(model) {
        return Some(p);
    }

    // Progressively strip the last `-` or `.` segment and retry.
    let mut candidate = model;
    while let Some(pos) = candidate.rfind(['-', '.']) {
        candidate = &candidate[..pos];
        if let Some(p) = models.get(candidate) {
            return Some(p);
        }
    }

    None
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_cost_known_model_returns_expected_value() {
        // gpt-4: input=0.00003, output=0.00006
        // 100 * 0.00003 + 50 * 0.00006 = 0.003 + 0.003 = 0.006
        let cost = completion_cost("gpt-4", 100, 50).expect("gpt-4 must be in registry");
        let expected = 100.0 * 0.00003 + 50.0 * 0.00006;
        assert!((cost - expected).abs() < 1e-12, "expected {expected}, got {cost}");
    }

    #[test]
    fn completion_cost_unknown_model_returns_none() {
        assert!(
            completion_cost("unknown-model-xyz", 100, 50).is_none(),
            "unknown model should return None"
        );
    }

    #[test]
    fn completion_cost_gpt4o_matches_published_pricing() {
        // gpt-4o: input=$2.50/1M tokens = 0.0000025/token
        //         output=$10/1M tokens  = 0.00001/token
        let cost = completion_cost("gpt-4o", 1_000, 500).expect("gpt-4o must be in registry");
        let expected = 1_000.0 * 0.0000025 + 500.0 * 0.00001;
        assert!((cost - expected).abs() < 1e-12, "expected {expected}, got {cost}");
    }

    #[test]
    fn completion_cost_embedding_model_has_zero_output_cost() {
        // Embedding models only charge for input tokens.
        let cost =
            completion_cost("text-embedding-3-small", 100, 0).expect("text-embedding-3-small must be in registry");
        assert!(cost > 0.0, "input tokens must have a positive cost");

        let pricing = model_pricing("text-embedding-3-small").unwrap();
        assert_eq!(pricing.output_cost_per_token, 0.0, "embedding output cost must be zero");
    }

    #[test]
    fn model_pricing_returns_none_for_unknown_model() {
        assert!(model_pricing("does-not-exist").is_none());
    }

    #[test]
    fn model_pricing_prefix_fallback_matches_shorter_name() {
        // gpt-4 is in the registry; gpt-4-0613 is a versioned variant that
        // should fall back to the gpt-4 entry via prefix stripping.
        let exact = model_pricing("gpt-4").expect("gpt-4 must be in registry");
        let prefix = model_pricing("gpt-4-0613").expect("gpt-4-0613 should match gpt-4 via prefix");
        assert!(
            (exact.input_cost_per_token - prefix.input_cost_per_token).abs() < 1e-15,
            "prefix match should return the same pricing as exact match"
        );
    }

    #[test]
    fn completion_cost_prefix_fallback() {
        // Versioned model name should resolve via prefix stripping.
        let cost = completion_cost("gpt-4-0613", 100, 50);
        assert!(cost.is_some(), "gpt-4-0613 should resolve via prefix fallback to gpt-4");
    }

    #[test]
    fn model_pricing_returns_correct_fields_for_known_model() {
        let p = model_pricing("gpt-4o-mini").expect("gpt-4o-mini must be in registry");
        // Published: input $0.15/1M = 0.00000015, output $0.60/1M = 0.0000006
        assert!(
            (p.input_cost_per_token - 0.00000015).abs() < 1e-12,
            "unexpected input_cost_per_token: {}",
            p.input_cost_per_token
        );
        assert!(
            (p.output_cost_per_token - 0.0000006).abs() < 1e-12,
            "unexpected output_cost_per_token: {}",
            p.output_cost_per_token
        );
    }

    #[test]
    fn completion_cost_with_cache_applies_discount_when_pricing_available() {
        // Use a synthetic pricing entry to make the math deterministic without
        // depending on upstream models.dev values.
        let pricing = ModelPricing {
            input_cost_per_token: 1e-5,
            output_cost_per_token: 2e-5,
            cache_read_input_token_cost: Some(1e-6),
            cache_creation_input_token_cost: None,
        };
        // 800 uncached @ 1e-5 + 200 cached @ 1e-6 + 50 output @ 2e-5
        let expected = 800.0 * 1e-5 + 200.0 * 1e-6 + 50.0 * 2e-5;
        let uncached = 1000 - 200;
        let actual = (uncached as f64) * pricing.input_cost_per_token
            + 200.0 * pricing.cache_read_input_token_cost.unwrap()
            + 50.0 * pricing.output_cost_per_token;
        assert!((actual - expected).abs() < 1e-12);
    }

    #[test]
    fn completion_cost_with_cache_falls_back_to_input_rate_without_cache_pricing() {
        // gpt-4 has no cache pricing in the embedded registry; the cached
        // portion should be billed at the regular input rate so the result
        // matches `completion_cost` ignoring the cache split.
        let with_cache = completion_cost_with_cache("gpt-4", 1_000, 200, 50).expect("gpt-4 must be in registry");
        let without = completion_cost("gpt-4", 1_000, 50).expect("gpt-4 must be in registry");
        assert!((with_cache - without).abs() < 1e-12);
    }

    #[test]
    fn completion_cost_with_cache_clamps_cached_tokens_to_prompt_tokens() {
        // Cached cannot exceed prompt; clamp instead of returning a negative
        // contribution from the uncached portion.
        let cost = completion_cost_with_cache("gpt-4", 100, 500, 0).expect("gpt-4 must be in registry");
        let clamped = completion_cost_with_cache("gpt-4", 100, 100, 0).expect("gpt-4 must be in registry");
        assert!((cost - clamped).abs() < 1e-12);
    }

    #[test]
    fn completion_cost_with_cache_uses_registry_cache_pricing_when_available() {
        // claude-sonnet-4-5 has cache_read pricing in the embedded registry.
        let pricing = model_pricing("claude-sonnet-4-5").expect("claude-sonnet-4-5 must be in registry");
        let cache_rate = pricing
            .cache_read_input_token_cost
            .expect("claude-sonnet-4-5 must have cache_read_input_token_cost");
        assert!(
            cache_rate < pricing.input_cost_per_token,
            "cache rate ({cache_rate}) must be cheaper than input rate ({})",
            pricing.input_cost_per_token
        );
        // 1000 prompt tokens, 200 cached, 50 output:
        // cost = 800 * input + 200 * cache_read + 50 * output
        let expected = 800.0 * pricing.input_cost_per_token + 200.0 * cache_rate + 50.0 * pricing.output_cost_per_token;
        let actual = completion_cost_with_cache("claude-sonnet-4-5", 1_000, 200, 50)
            .expect("claude-sonnet-4-5 must be priceable");
        assert!((actual - expected).abs() < 1e-12, "expected {expected}, got {actual}");
        // Sanity: cached cost is strictly cheaper than billing all tokens at the
        // input rate, which is the user-visible point of the feature.
        let no_cache = completion_cost("claude-sonnet-4-5", 1_000, 50).expect("claude-sonnet-4-5 must be priceable");
        assert!(
            actual < no_cache,
            "cached cost ({actual}) must be < uncached ({no_cache})"
        );
    }

    #[test]
    fn completion_cost_with_cache_unknown_model_returns_none() {
        assert!(completion_cost_with_cache("unknown-model-xyz", 100, 10, 50).is_none());
    }

    #[test]
    fn pricing_registry_embedded_json_is_valid() {
        // Confirm the embedded JSON parses correctly — PRICING holds Ok(...).
        assert!(
            PRICING.as_ref().is_ok(),
            "embedded schemas/pricing.json failed to parse: {:?}",
            PRICING.as_ref().err()
        );
    }
}
