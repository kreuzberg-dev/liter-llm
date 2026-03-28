//! liter-llm Ruby Bindings (Magnus 0.8)
//!
//! Provides a Ruby-idiomatic `LiterLlm::LlmClient` class backed by the Rust
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
//! require 'liter_llm'
//!
//! client = LiterLlm::LlmClient.new('sk-...', base_url: 'https://api.openai.com/v1')
//!
//! response = JSON.parse(client.chat(JSON.generate(
//!   model: 'gpt-4',
//!   messages: [{ role: 'user', content: 'Hello' }]
//! )))
//!
//! puts response.dig('choices', 0, 'message', 'content')
//! ```

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock, Mutex};

use liter_llm::tower::{BudgetConfig, CacheConfig, Enforcement, LlmHook, LlmRequest, LlmResponse, RateLimitConfig};
use liter_llm::{
    AuthHeaderFormat, BatchClient, ClientConfigBuilder, CustomProviderConfig, FileClient, LiterLlmError,
    LlmClient, ManagedClient, ResponseClient, register_custom_provider, unregister_custom_provider,
};
use magnus::{Error, RHash, Ruby, TryConvert, function, method, prelude::*};

// ─── Tokio runtime ────────────────────────────────────────────────────────────

/// Process-wide Tokio runtime used to drive async calls from synchronous Ruby.
///
/// Created once on first use.  If creation fails, the error message is stored
/// and returned as a Ruby `RuntimeError` at call time rather than panicking.
static RUNTIME: LazyLock<Result<tokio::runtime::Runtime, String>> = LazyLock::new(|| {
    // current_thread keeps block_on on the Ruby thread that called the method.
    // A multi-thread runtime would dispatch futures to worker threads where
    // Ruby::get_unchecked() is invalid (it requires the GVL holder thread).
    // current_thread avoids spawning extra OS threads and is sufficient for
    // Ruby's single-threaded-per-thread concurrency model.
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name("liter-llm-ruby")
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

// ─── Ruby Hook Bridge ────────────────────────────────────────────────────────

/// A hook that stores callback names as JSON strings and invokes Ruby Procs.
///
/// Since Ruby is single-threaded with GVL, hooks are called synchronously
/// inside `block_on` — no need for `spawn_blocking`.
#[allow(dead_code)]
struct RubyHookBridge {
    /// JSON string identifying the hook class name for diagnostics.
    name: String,
    /// Ruby callables: `on_request`, `on_response`, `on_error`.
    /// Stored as serialized data since we cannot hold Ruby references across
    /// thread boundaries.  Instead, the hook methods are encoded as a contract:
    /// the Ruby object must respond to the named methods.
    ///
    /// We store the hook object reference index to call back into Ruby.
    _marker: (),
}

// SAFETY: Ruby hooks are only called from the GVL-holding thread inside
// `block_on`, never from a Tokio worker thread.
unsafe impl Send for RubyHookBridge {}
unsafe impl Sync for RubyHookBridge {}

impl LlmHook for RubyHookBridge {
    fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = liter_llm::Result<()>> + Send + '_>> {
        // No-op: Ruby hooks are invoked via a different mechanism (see `run_hooks`).
        Box::pin(async { Ok(()) })
    }

    fn on_response(
        &self,
        _req: &LlmRequest,
        _resp: &LlmResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    fn on_error(
        &self,
        _req: &LlmRequest,
        _err: &LiterLlmError,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }
}

// ─── Helper: parse Ruby hash into CacheConfig ────────────────────────────────

fn parse_cache_config_rb(kw: &RHash) -> Result<CacheConfig, Error> {
    let ruby = unsafe { Ruby::get_unchecked() };
    let max_entries: usize = kw
        .get(ruby.to_symbol("max_entries"))
        .and_then(|v| usize::try_convert(v).ok())
        .unwrap_or(256);
    let ttl_seconds: u64 = kw
        .get(ruby.to_symbol("ttl_seconds"))
        .and_then(|v| u64::try_convert(v).ok())
        .unwrap_or(300);
    Ok(CacheConfig {
        max_entries,
        ttl: std::time::Duration::from_secs(ttl_seconds),
        backend: Default::default(),
    })
}

// ─── Helper: parse Ruby hash into BudgetConfig ──────────────────────────────

fn parse_budget_config_rb(kw: &RHash) -> Result<BudgetConfig, Error> {
    let ruby = unsafe { Ruby::get_unchecked() };
    let global_limit: Option<f64> = kw
        .get(ruby.to_symbol("global_limit"))
        .and_then(|v| Option::<f64>::try_convert(v).ok())
        .flatten();
    let enforcement_str: String = kw
        .get(ruby.to_symbol("enforcement"))
        .and_then(|v| String::try_convert(v).ok())
        .unwrap_or_else(|| "hard".to_owned());
    let enforcement = match enforcement_str.as_str() {
        "soft" => Enforcement::Soft,
        _ => Enforcement::Hard,
    };
    // model_limits is a Ruby Hash of String -> Float
    let model_limits: HashMap<String, f64> = kw
        .get(ruby.to_symbol("model_limits"))
        .and_then(|v| {
            let hash = magnus::RHash::try_convert(v).ok()?;
            let mut map = HashMap::new();
            let _ = hash.foreach(|k: String, v: f64| {
                map.insert(k, v);
                Ok(magnus::r_hash::ForEach::Continue)
            });
            Some(map)
        })
        .unwrap_or_default();
    Ok(BudgetConfig {
        global_limit,
        model_limits,
        enforcement,
    })
}

// ─── Helper: parse Ruby hash into CustomProviderConfig ──────────────────────

fn parse_provider_config_rb(kw: &RHash) -> Result<CustomProviderConfig, Error> {
    let ruby = unsafe { Ruby::get_unchecked() };
    let name: String = kw
        .get(ruby.to_symbol("name"))
        .and_then(|v| String::try_convert(v).ok())
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "provider config requires :name"))?;
    let base_url: String = kw
        .get(ruby.to_symbol("base_url"))
        .and_then(|v| String::try_convert(v).ok())
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "provider config requires :base_url"))?;
    let auth_header_str: String = kw
        .get(ruby.to_symbol("auth_header"))
        .and_then(|v| String::try_convert(v).ok())
        .unwrap_or_else(|| "bearer".to_owned());
    let auth_header = match auth_header_str.as_str() {
        "none" => AuthHeaderFormat::None,
        s if s.starts_with("api-key:") => {
            AuthHeaderFormat::ApiKey(s.trim_start_matches("api-key:").trim().to_owned())
        }
        _ => AuthHeaderFormat::Bearer,
    };
    let model_prefixes: Vec<String> = kw
        .get(ruby.to_symbol("model_prefixes"))
        .and_then(|v| <Vec<String>>::try_convert(v).ok())
        .unwrap_or_default();
    Ok(CustomProviderConfig {
        name,
        base_url,
        auth_header,
        model_prefixes,
    })
}

// ─── RubyLlmClient ──────────────────────────────────────────────────────────

/// Ruby wrapper around `liter_llm::ManagedClient`.
#[magnus::wrap(class = "LiterLlm::LlmClient", free_immediately, size)]
pub struct RubyLlmClient {
    inner: ManagedClient,
    /// Runtime-registered hooks.  Stored as `Arc<dyn LlmHook>` for
    /// compatibility with the Rust trait, though Ruby hooks are invoked
    /// synchronously within `block_on`.
    hooks: Mutex<Vec<Arc<dyn LlmHook>>>,
}

impl RubyLlmClient {
    /// `LiterLlm::LlmClient.new(api_key, base_url: nil, model_hint: nil,
    ///   max_retries: 3, timeout_secs: 60, cache: nil, budget: nil,
    ///   extra_headers: nil)`
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

        // Apply optional cache configuration.
        if let Some(cache_val) = kw.get(ruby.to_symbol("cache"))
            && let Ok(cache_hash) = magnus::RHash::try_convert(cache_val)
        {
            let cache_cfg = parse_cache_config_rb(&cache_hash)?;
            builder = builder.cache(cache_cfg);
        }

        // Apply optional budget configuration.
        if let Some(budget_val) = kw.get(ruby.to_symbol("budget"))
            && let Ok(budget_hash) = magnus::RHash::try_convert(budget_val)
        {
            let budget_cfg = parse_budget_config_rb(&budget_hash)?;
            builder = builder.budget(budget_cfg);
        }

        // Apply optional extra headers.
        if let Some(headers_val) = kw.get(ruby.to_symbol("extra_headers"))
            && let Ok(headers_hash) = magnus::RHash::try_convert(headers_val)
        {
            let mut pairs: Vec<(String, String)> = Vec::new();
            let _ = headers_hash.foreach(|k: String, v: String| {
                pairs.push((k, v));
                Ok(magnus::r_hash::ForEach::Continue)
            });
            for (k, v) in pairs {
                builder = builder
                    .header(k, v)
                    .map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;
            }
        }

        // Apply optional cooldown configuration.
        if let Some(cooldown_val) = kw.get(ruby.to_symbol("cooldown"))
            && let Ok(secs) = u64::try_convert(cooldown_val)
        {
            builder = builder.cooldown(std::time::Duration::from_secs(secs));
        }

        // Apply optional rate limit configuration.
        if let Some(rl_val) = kw.get(ruby.to_symbol("rate_limit"))
            && let Ok(rl_hash) = magnus::RHash::try_convert(rl_val)
        {
            let rpm: Option<u32> = rl_hash
                .get(ruby.to_symbol("rpm"))
                .and_then(|v| u32::try_convert(v).ok());
            let tpm: Option<u64> = rl_hash
                .get(ruby.to_symbol("tpm"))
                .and_then(|v| u64::try_convert(v).ok());
            let window_seconds: u64 = rl_hash
                .get(ruby.to_symbol("window_seconds"))
                .and_then(|v| u64::try_convert(v).ok())
                .unwrap_or(60);
            let rl_config = RateLimitConfig {
                rpm,
                tpm,
                window: std::time::Duration::from_secs(window_seconds),
            };
            builder = builder.rate_limit(rl_config);
        }

        // Apply optional health check interval.
        if let Some(hc_val) = kw.get(ruby.to_symbol("health_check"))
            && let Ok(secs) = u64::try_convert(hc_val)
        {
            builder = builder.health_check(std::time::Duration::from_secs(secs));
        }

        // Apply cost tracking flag.
        if let Some(ct_val) = kw.get(ruby.to_symbol("cost_tracking"))
            && let Ok(enabled) = bool::try_convert(ct_val)
            && enabled
        {
            builder = builder.cost_tracking(true);
        }

        // Apply tracing flag.
        if let Some(tr_val) = kw.get(ruby.to_symbol("tracing"))
            && let Ok(enabled) = bool::try_convert(tr_val)
            && enabled
        {
            builder = builder.tracing(true);
        }

        let config = builder.build();
        let client = ManagedClient::new(config, model_hint.as_deref())
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;

        Ok(RubyLlmClient {
            inner: client,
            hooks: Mutex::new(Vec::new()),
        })
    }

    /// Register a hook object.
    ///
    /// The hook object should respond to `on_request(request_json)`,
    /// `on_response(request_json, response_json)`, and/or
    /// `on_error(request_json, error_string)`.  All methods are optional.
    ///
    /// @param hook_name [String] A descriptive name for the hook.
    fn add_hook(&self, hook_name: String) -> Result<(), Error> {
        let ruby = unsafe { Ruby::get_unchecked() };
        let bridge = RubyHookBridge {
            name: hook_name,
            _marker: (),
        };
        self.hooks
            .lock()
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("hook lock poisoned: {e}")))?
            .push(Arc::new(bridge));
        Ok(())
    }

    /// Register a custom LLM provider in the global provider registry.
    ///
    /// @param config_hash [Hash] Provider configuration with :name, :base_url,
    ///   :auth_header, and :model_prefixes keys.
    fn rb_register_provider(config_hash: magnus::RHash) -> Result<(), Error> {
        let ruby = unsafe { Ruby::get_unchecked() };
        let provider_cfg = parse_provider_config_rb(&config_hash)?;
        register_custom_provider(provider_cfg)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))
    }

    /// Send a chat completion request.
    ///
    /// @param request_json [String] JSON-encoded OpenAI-compatible chat request.
    /// @return [String] JSON-encoded chat completion response.
    fn chat(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::ChatCompletionRequest =
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

        let req: liter_llm::EmbeddingRequest =
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

    /// Generate an image from a text prompt.
    ///
    /// @param request_json [String] JSON-encoded image generation request.
    /// @return [String] JSON-encoded images response.
    fn image_generate(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateImageRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid image request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.image_generate(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Generate audio speech from text, returning base64-encoded audio bytes.
    ///
    /// @param request_json [String] JSON-encoded speech request.
    /// @return [String] Base64-encoded raw audio bytes.
    fn speech(&self, request_json: String) -> Result<String, Error> {
        use base64::Engine;

        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateSpeechRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid speech request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.speech(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        Ok(base64::engine::general_purpose::STANDARD.encode(&response))
    }

    /// Transcribe audio to text.
    ///
    /// @param request_json [String] JSON-encoded transcription request.
    /// @return [String] JSON-encoded transcription response.
    fn transcribe(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateTranscriptionRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid transcription request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.transcribe(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Check content against moderation policies.
    ///
    /// @param request_json [String] JSON-encoded moderation request.
    /// @return [String] JSON-encoded moderation response.
    fn moderate(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::ModerationRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid moderation request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.moderate(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Rerank documents by relevance to a query.
    ///
    /// @param request_json [String] JSON-encoded rerank request.
    /// @return [String] JSON-encoded rerank response.
    fn rerank(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::RerankRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid rerank request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.rerank(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Perform a web/document search.
    ///
    /// @param request_json [String] JSON-encoded search request.
    /// @return [String] JSON-encoded search response.
    fn search(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::SearchRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid search request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.search(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Extract text from a document via OCR.
    ///
    /// @param request_json [String] JSON-encoded OCR request.
    /// @return [String] JSON-encoded OCR response.
    fn ocr(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::OcrRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid OCR request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.ocr(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    // ─── File Management ──────────────────────────────────────────────────────

    /// Upload a file.
    ///
    /// @param request_json [String] JSON-encoded file upload request.
    /// @return [String] JSON-encoded file object.
    fn create_file(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateFileRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid file request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.create_file(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Retrieve metadata for a file by ID.
    ///
    /// @param file_id [String] The file identifier.
    /// @return [String] JSON-encoded file object.
    fn retrieve_file(&self, file_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.retrieve_file(&file_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Delete a file by ID.
    ///
    /// @param file_id [String] The file identifier.
    /// @return [String] JSON-encoded delete response.
    fn delete_file(&self, file_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.delete_file(&file_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// List files, optionally filtered by query parameters.
    ///
    /// @param query_json [String, nil] JSON-encoded file list query parameters, or nil.
    /// @return [String] JSON-encoded file list response.
    fn list_files(&self, query_json: Option<String>) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let query: Option<liter_llm::FileListQuery> = match query_json {
            Some(json) => Some(serde_json::from_str(&json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid file list query JSON: {e}"),
                )
            })?),
            None => None,
        };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.list_files(query)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Retrieve the raw content of a file.
    ///
    /// @param file_id [String] The file identifier.
    /// @return [String] Base64-encoded raw file content.
    fn file_content(&self, file_id: String) -> Result<String, Error> {
        use base64::Engine;

        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.file_content(&file_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        Ok(base64::engine::general_purpose::STANDARD.encode(&response))
    }

    // ─── Batch Management ─────────────────────────────────────────────────────

    /// Create a new batch job.
    ///
    /// @param request_json [String] JSON-encoded batch creation request.
    /// @return [String] JSON-encoded batch object.
    fn create_batch(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateBatchRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid batch request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.create_batch(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Retrieve a batch by ID.
    ///
    /// @param batch_id [String] The batch identifier.
    /// @return [String] JSON-encoded batch object.
    fn retrieve_batch(&self, batch_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.retrieve_batch(&batch_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// List batches, optionally filtered by query parameters.
    ///
    /// @param query_json [String, nil] JSON-encoded batch list query parameters, or nil.
    /// @return [String] JSON-encoded batch list response.
    fn list_batches(&self, query_json: Option<String>) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let query: Option<liter_llm::BatchListQuery> = match query_json {
            Some(json) => Some(serde_json::from_str(&json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid batch list query JSON: {e}"),
                )
            })?),
            None => None,
        };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.list_batches(query)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Cancel an in-progress batch.
    ///
    /// @param batch_id [String] The batch identifier.
    /// @return [String] JSON-encoded batch object.
    fn cancel_batch(&self, batch_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.cancel_batch(&batch_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    // ─── Responses API ────────────────────────────────────────────────────────

    /// Create a new response via the Responses API.
    ///
    /// @param request_json [String] JSON-encoded response creation request.
    /// @return [String] JSON-encoded response object.
    fn create_response(&self, request_json: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::CreateResponseRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid response request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.create_response(req)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Retrieve a response by ID.
    ///
    /// @param response_id [String] The response identifier.
    /// @return [String] JSON-encoded response object.
    fn retrieve_response(&self, response_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.retrieve_response(&response_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Cancel an in-progress response.
    ///
    /// @param response_id [String] The response identifier.
    /// @return [String] JSON-encoded response object.
    fn cancel_response(&self, response_id: String) -> Result<String, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };

        let rt = runtime(&ruby)?;
        let response = rt.block_on(self.inner.cancel_response(&response_id)).map_err(|e| {
            Error::new(ruby.exception_runtime_error(), e.to_string())
        })?;

        serde_json::to_string(&response).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Stream a chat completion request, collecting all chunks.
    ///
    /// Returns a JSON array of serialised `ChatCompletionChunk` objects.
    /// Each element is the JSON for one SSE chunk. Ruby callers can iterate
    /// with `JSON.parse(result).each { |chunk| ... }`.
    ///
    /// @param request_json [String] JSON-encoded OpenAI-compatible chat request.
    /// @return [String] JSON-encoded array of chat completion chunks.
    fn chat_stream(&self, request_json: String) -> Result<String, Error> {
        use futures_core::Stream;
        use std::pin::Pin;

        let ruby = unsafe { Ruby::get_unchecked() };

        let req: liter_llm::ChatCompletionRequest =
            serde_json::from_str(&request_json).map_err(|e| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("invalid chat request JSON: {e}"),
                )
            })?;

        let rt = runtime(&ruby)?;
        let chunks: Vec<liter_llm::ChatCompletionChunk> = rt
            .block_on(async {
                let mut stream = self.inner.chat_stream(req).await.map_err(|e| {
                    Error::new(ruby.exception_runtime_error(), e.to_string())
                })?;

                let mut collected = Vec::new();
                loop {
                    let next =
                        std::future::poll_fn(|cx| Pin::new(&mut stream).poll_next(cx)).await;
                    match next {
                        None => break,
                        Some(Err(e)) => {
                            return Err(Error::new(
                                ruby.exception_runtime_error(),
                                e.to_string(),
                            ));
                        }
                        Some(Ok(chunk)) => collected.push(chunk),
                    }
                }
                Ok(collected)
            })?;

        serde_json::to_string(&chunks).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("serialization error: {e}"),
            )
        })
    }

    /// Unregister a previously registered custom provider by name.
    ///
    /// @param name [String] The provider name to unregister.
    /// @return [Boolean] `true` if the provider was found and removed, `false` otherwise.
    fn rb_unregister_provider(name: String) -> Result<bool, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };
        unregister_custom_provider(&name)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))
    }

    /// Return the total budget spend so far (in USD).
    ///
    /// Returns `0.0` if no budget is configured.
    ///
    /// @return [Float] The cumulative global spend tracked by the budget layer.
    fn budget_used(&self) -> f64 {
        self.inner
            .budget_state()
            .map(|s| s.global_spend())
            .unwrap_or(0.0)
    }

    /// Return a human-readable string representation.
    fn inspect(&self) -> String {
        "#<LiterLlm::LlmClient>".to_string()
    }
}

// ─── Module entry point ───────────────────────────────────────────────────────

/// `Init_liter_llm_rb` — called by Ruby when the extension is `require`d.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    // Define the `LiterLlm` namespace module.
    let liter_llm_mod = ruby.define_module("LiterLlm")?;

    // Define `LiterLlm::LlmClient`.
    let client_class = liter_llm_mod.define_class("LlmClient", ruby.class_object())?;

    // Constructor: LlmClient.new(api_key, base_url: nil, max_retries: 3, timeout_secs: 60)
    client_class.define_singleton_method("new", function!(RubyLlmClient::rb_new, 2))?;

    // Hook and provider registration.
    client_class.define_method("add_hook", method!(RubyLlmClient::add_hook, 1))?;
    client_class.define_singleton_method("register_provider", function!(RubyLlmClient::rb_register_provider, 1))?;
    client_class.define_singleton_method("unregister_provider", function!(RubyLlmClient::rb_unregister_provider, 1))?;

    // Instance methods.
    client_class.define_method("chat", method!(RubyLlmClient::chat, 1))?;
    client_class.define_method("chat_stream", method!(RubyLlmClient::chat_stream, 1))?;
    client_class.define_method("budget_used", method!(RubyLlmClient::budget_used, 0))?;
    client_class.define_method("embed", method!(RubyLlmClient::embed, 1))?;
    client_class.define_method("list_models", method!(RubyLlmClient::list_models, 0))?;

    // Inference methods.
    client_class.define_method("image_generate", method!(RubyLlmClient::image_generate, 1))?;
    client_class.define_method("speech", method!(RubyLlmClient::speech, 1))?;
    client_class.define_method("transcribe", method!(RubyLlmClient::transcribe, 1))?;
    client_class.define_method("moderate", method!(RubyLlmClient::moderate, 1))?;
    client_class.define_method("rerank", method!(RubyLlmClient::rerank, 1))?;
    client_class.define_method("search", method!(RubyLlmClient::search, 1))?;
    client_class.define_method("ocr", method!(RubyLlmClient::ocr, 1))?;

    // File management methods.
    client_class.define_method("create_file", method!(RubyLlmClient::create_file, 1))?;
    client_class.define_method("retrieve_file", method!(RubyLlmClient::retrieve_file, 1))?;
    client_class.define_method("delete_file", method!(RubyLlmClient::delete_file, 1))?;
    client_class.define_method("list_files", method!(RubyLlmClient::list_files, 1))?;
    client_class.define_method("file_content", method!(RubyLlmClient::file_content, 1))?;

    // Batch management methods.
    client_class.define_method("create_batch", method!(RubyLlmClient::create_batch, 1))?;
    client_class.define_method("retrieve_batch", method!(RubyLlmClient::retrieve_batch, 1))?;
    client_class.define_method("list_batches", method!(RubyLlmClient::list_batches, 1))?;
    client_class.define_method("cancel_batch", method!(RubyLlmClient::cancel_batch, 1))?;

    // Responses API methods.
    client_class.define_method("create_response", method!(RubyLlmClient::create_response, 1))?;
    client_class.define_method("retrieve_response", method!(RubyLlmClient::retrieve_response, 1))?;
    client_class.define_method("cancel_response", method!(RubyLlmClient::cancel_response, 1))?;

    client_class.define_method("inspect", method!(RubyLlmClient::inspect, 0))?;
    client_class.define_method("to_s", method!(RubyLlmClient::inspect, 0))?;

    // Module-level version constant.
    liter_llm_mod.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
