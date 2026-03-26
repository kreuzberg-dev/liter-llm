//! liter-lm Ruby Bindings (Magnus 0.8)
//!
//! Provides a Ruby-idiomatic `LiterLm::LlmClient` class backed by the Rust
//! core library.
//!
//! # Architecture
//!
//! Ruby (MRI) is single-threaded with a GVL.  Async Rust futures are driven to
//! completion with `tokio::runtime::Runtime::block_on` inside each method.  A
//! single Tokio runtime lives for the process lifetime, created lazily the
//! first time any method is called.
//!
//! All request/response parameters are accepted and returned as JSON strings.
//! Ruby callers use `JSON.parse` / `JSON.generate`.
//!
//! # Example (Ruby)
//!
//! ```ruby
//! require 'liter_lm'
//!
//! client = LiterLm::LlmClient.new('sk-...', base_url: 'https://api.openai.com/v1')
//!
//! response = JSON.parse(client.chat(JSON.generate(
//!   model: 'gpt-4',
//!   messages: [{ role: 'user', content: 'Hello' }]
//! )))
//!
//! puts response.dig('choices', 0, 'message', 'content')
//! ```

use std::sync::LazyLock;

use liter_lm::{ClientConfigBuilder, DefaultClient, LlmClient};
use magnus::{Error, Ruby, TryConvert, function, method, prelude::*};

// ─── Tokio runtime ────────────────────────────────────────────────────────────

/// Process-wide Tokio runtime used to drive async calls from synchronous Ruby.
///
/// Created once on first use.  If creation fails, the error message is stored
/// and returned as a Ruby `RuntimeError` at call time rather than panicking.
static RUNTIME: LazyLock<Result<tokio::runtime::Runtime, String>> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("liter-lm-ruby")
        .build()
        .map_err(|e| format!("Failed to create Tokio runtime: {e}"))
});

/// Return a reference to the shared runtime, or a Ruby `RuntimeError`.
fn runtime(ruby: &Ruby) -> Result<&'static tokio::runtime::Runtime, Error> {
    RUNTIME
        .as_ref()
        .map_err(|e| Error::new(ruby.exception_runtime_error(), e.clone()))
}

// ─── RubyLlmClient ────────────────────────────────────────────────────────────

/// Ruby wrapper around `liter_lm::DefaultClient`.
#[magnus::wrap(class = "LiterLm::LlmClient", free_immediately, size)]
pub struct RubyLlmClient {
    inner: DefaultClient,
}

impl RubyLlmClient {
    /// `LiterLm::LlmClient.new(api_key, base_url: nil, model_hint: nil, max_retries: 3, timeout_secs: 60)`
    ///
    /// Takes an API key string and an optional keyword-argument hash.
    fn rb_new(api_key: String, kw: magnus::RHash) -> Result<RubyLlmClient, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let base_url: Option<String> = kw
            .get(ruby.to_symbol("base_url"))
            .and_then(|v| Option::<String>::try_convert(v).ok())
            .flatten();

        let model_hint: Option<String> = kw
            .get(ruby.to_symbol("model_hint"))
            .and_then(|v| Option::<String>::try_convert(v).ok())
            .flatten();

        let max_retries: u32 = kw
            .get(ruby.to_symbol("max_retries"))
            .and_then(|v| u32::try_convert(v).ok())
            .unwrap_or(3);

        let timeout_secs: u64 = kw
            .get(ruby.to_symbol("timeout_secs"))
            .and_then(|v| u64::try_convert(v).ok())
            .unwrap_or(60);

        let mut builder = ClientConfigBuilder::new(api_key);
        if let Some(url) = base_url {
            builder = builder.base_url(url);
        }
        builder = builder.max_retries(max_retries);
        builder = builder.timeout(std::time::Duration::from_secs(timeout_secs));

        let config = builder.build();
        let client = DefaultClient::new(config, model_hint.as_deref())
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;

        Ok(RubyLlmClient { inner: client })
    }

    /// Send a chat completion request.
    ///
    /// @param request_json [String] JSON-encoded OpenAI-compatible chat request.
    /// @return [String] JSON-encoded chat completion response.
    fn chat(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_lm::ChatCompletionRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid chat request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.chat(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Send an embedding request.
    ///
    /// @param request_json [String] JSON-encoded OpenAI-compatible embeddings request.
    /// @return [String] JSON-encoded embedding response.
    fn embed(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_lm::EmbeddingRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid embed request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.embed(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// List available models from the provider.
    ///
    /// @return [String] JSON-encoded models list response.
    fn list_models(&self) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.list_models()).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Return a human-readable string representation.
    fn inspect(&self) -> String {
        "#<LiterLm::LlmClient>".to_string()
    }
}

// ─── Module entry point ───────────────────────────────────────────────────────

/// `Init_liter_lm_rb` — called by Ruby when the extension is `require`d.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    // Define the `LiterLm` namespace module.
    let liter_lm_mod = ruby.define_module("LiterLm")?;

    // Define `LiterLm::LlmClient`.
    let client_class = liter_lm_mod.define_class("LlmClient", ruby.class_object())?;

    // Constructor: LlmClient.new(api_key, base_url: nil, max_retries: 3, timeout_secs: 60)
    client_class.define_singleton_method("new", function!(RubyLlmClient::rb_new, 2))?;

    // Instance methods.
    client_class.define_method("chat", method!(RubyLlmClient::chat, 1))?;
    client_class.define_method("embed", method!(RubyLlmClient::embed, 1))?;
    client_class.define_method("list_models", method!(RubyLlmClient::list_models, 0))?;
    client_class.define_method("inspect", method!(RubyLlmClient::inspect, 0))?;
    client_class.define_method("to_s", method!(RubyLlmClient::inspect, 0))?;

    // Module-level version constant.
    liter_lm_mod.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
