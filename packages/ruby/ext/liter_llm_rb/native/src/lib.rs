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

use std::sync::LazyLock;

use liter_llm::{BatchClient, ClientConfigBuilder, DefaultClient, FileClient, LlmClient, ResponseClient};
use magnus::{Error, Ruby, TryConvert, function, method, prelude::*};

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

/// Ruby wrapper around `liter_llm::DefaultClient`.
#[magnus::wrap(class = "LiterLlm::LlmClient", free_immediately, size)]
pub struct RubyLlmClient {
    inner: DefaultClient,
}

impl RubyLlmClient {
    /// `LiterLlm::LlmClient.new(api_key, base_url: nil, model_hint: nil, max_retries: 3, timeout_secs: 60)`
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

    // Instance methods.
    client_class.define_method("chat", method!(RubyLlmClient::chat, 1))?;
    client_class.define_method("embed", method!(RubyLlmClient::embed, 1))?;
    client_class.define_method("list_models", method!(RubyLlmClient::list_models, 0))?;

    // Inference methods.
    client_class.define_method("image_generate", method!(RubyLlmClient::image_generate, 1))?;
    client_class.define_method("speech", method!(RubyLlmClient::speech, 1))?;
    client_class.define_method("transcribe", method!(RubyLlmClient::transcribe, 1))?;
    client_class.define_method("moderate", method!(RubyLlmClient::moderate, 1))?;
    client_class.define_method("rerank", method!(RubyLlmClient::rerank, 1))?;

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
