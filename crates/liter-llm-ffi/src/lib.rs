//! C FFI bindings for liter-llm.
//!
//! Provides an opaque-handle C API consumed by Go (cgo), Java (Panama FFM),
//! C# (P/Invoke), and any other language with C FFI support.
//!
//! ## Ownership model
//!
//! - [`literllm_client_new`] returns a heap-allocated `*mut LiterLlmClient`.
//!   The caller **owns** it and must eventually call [`literllm_client_free`].
//! - [`literllm_chat`], [`literllm_embed`], [`literllm_list_models`] return
//!   heap-allocated `*mut c_char` JSON strings.
//!   The caller **owns** them and must call [`literllm_free_string`].
//! - [`literllm_last_error`] returns a thread-local `*const c_char`.
//!   The caller must **not** free it; it is valid until the next call on the
//!   same thread.

use std::ffi::{CStr, CString, c_char};

use liter_llm::client::{BatchClient, ClientConfig, DefaultClient, FileClient, LlmClient, ResponseClient};

// ---------------------------------------------------------------------------
// Thread-local last-error storage
// ---------------------------------------------------------------------------

thread_local! {
    /// Holds the last error message for the current thread.
    /// Stored as a `CString` so the pointer stays valid until next error.
    static LAST_ERROR: std::cell::RefCell<Option<CString>> =
        const { std::cell::RefCell::new(None) };
}

/// Store a new last-error string for this thread.
fn set_last_error(msg: String) {
    LAST_ERROR.with(|cell| {
        // Silently fall back to a truncated message if the string contains
        // interior NUL bytes (should never happen in practice).
        let c_str = CString::new(msg).unwrap_or_else(|_| c"<error message contained NUL byte>".into());
        *cell.borrow_mut() = Some(c_str);
    });
}

/// Clear the last-error for this thread.
fn clear_last_error() {
    LAST_ERROR.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

// ---------------------------------------------------------------------------
// Opaque client handle
// ---------------------------------------------------------------------------

/// Opaque handle to a liter-llm client.
///
/// Create with [`literllm_client_new`], destroy with [`literllm_client_free`].
/// All fields are private; callers interact only through the public functions.
///
/// cbindgen:no-export — we emit the opaque declaration manually in the header
/// preamble so C callers only ever hold a `LiterLlmClient*`.
pub struct LiterLlmClient {
    inner: DefaultClient,
}

/// Tokio runtime used for blocking on async operations from synchronous C callers.
///
/// A single runtime is created on first use and shared across all threads.
///
/// # Thread safety
///
/// `LiterLlmClient` holds a `DefaultClient`, which is `Send + Sync`.  The
/// shared runtime is likewise `Send + Sync`.  All calls into this crate are
/// therefore safe to make from multiple threads concurrently.
// Compile-time assertion: DefaultClient must be Send + Sync so that the
// opaque handle can be used from multiple C threads without data races.
const _: () = {
    const fn _assert_send_sync<T: Send + Sync>() {}
    // Called at compile time — zero run-time cost.
    let _ = _assert_send_sync::<DefaultClient>;
};

fn runtime() -> Result<&'static tokio::runtime::Runtime, String> {
    static RT: std::sync::OnceLock<Result<tokio::runtime::Runtime, String>> = std::sync::OnceLock::new();
    RT.get_or_init(|| {
        // Use current_thread so that block_on drives all work on the calling
        // thread.  This guarantees that LAST_ERROR TLS writes happen on the
        // same thread that called the public API function, which is essential
        // for correctness: if a multi-thread runtime dispatched work to a
        // worker thread the caller's LAST_ERROR cell would never be updated.
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("failed to build tokio runtime: {e}"))
    })
    .as_ref()
    .map_err(|e| e.clone())
}

// ---------------------------------------------------------------------------
// Public C API
// ---------------------------------------------------------------------------

/// Create a new liter-llm client.
///
/// # Parameters
///
/// - `api_key`: NUL-terminated API key string.  Pass an empty string (`""`)
///   when using a provider that does not require authentication.
/// - `base_url`: NUL-terminated base URL override.  Pass `NULL` to use the
///   default provider routing based on model-name prefix.
/// - `model_hint`: NUL-terminated model name hint for provider auto-detection
///   (e.g. `"groq/llama3-70b"`).  Pass `NULL` to default to OpenAI.  Used
///   only when `base_url` is also `NULL`.
///
/// # Return value
///
/// Returns a heap-allocated `LiterLlmClient*` on success, or `NULL` on failure.
/// Check [`literllm_last_error`] for the error message when `NULL` is returned.
///
/// The returned pointer must be freed with [`literllm_client_free`].
///
/// # Safety
///
/// - `api_key` must be a valid, non-null, NUL-terminated C string.
/// - `base_url` may be `NULL` (treated as no override) or a valid NUL-terminated C string.
/// - `model_hint` may be `NULL` (treated as no hint) or a valid NUL-terminated C string.
/// - The caller owns the returned pointer and must call `literllm_client_free` exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_client_new(
    api_key: *const c_char,
    base_url: *const c_char,
    model_hint: *const c_char,
) -> *mut LiterLlmClient {
    clear_last_error();

    // SAFETY: caller guarantees `api_key` is non-null and NUL-terminated.
    if api_key.is_null() {
        set_last_error("literllm_client_new: api_key must not be NULL".into());
        return std::ptr::null_mut();
    }

    let api_key_str = match unsafe { CStr::from_ptr(api_key) }.to_str() {
        Ok(s) => s.to_owned(),
        Err(e) => {
            set_last_error(format!("literllm_client_new: api_key is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let mut config_builder = liter_llm::client::ClientConfigBuilder::new(api_key_str);

    // SAFETY: `base_url` is either NULL (skip) or a valid NUL-terminated C string.
    if !base_url.is_null() {
        match unsafe { CStr::from_ptr(base_url) }.to_str() {
            Ok(url) if !url.is_empty() => {
                config_builder = config_builder.base_url(url);
            }
            Ok(_) => {} // empty string — treat as no override
            Err(e) => {
                set_last_error(format!("literllm_client_new: base_url is not valid UTF-8: {e}"));
                return std::ptr::null_mut();
            }
        }
    }

    // Parse model_hint: NULL or empty string → None; otherwise Some(&str).
    // SAFETY: `model_hint` is either NULL (skip) or a valid NUL-terminated C string.
    let model_hint_str: Option<String> = if model_hint.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(model_hint) }.to_str() {
            Ok(s) if !s.is_empty() => Some(s.to_owned()),
            Ok(_) => None, // empty string — treat as no hint
            Err(e) => {
                set_last_error(format!("literllm_client_new: model_hint is not valid UTF-8: {e}"));
                return std::ptr::null_mut();
            }
        }
    };

    let config: ClientConfig = config_builder.build();

    match DefaultClient::new(config, model_hint_str.as_deref()) {
        Ok(client) => {
            let handle = Box::new(LiterLlmClient { inner: client });
            Box::into_raw(handle)
        }
        Err(e) => {
            set_last_error(format!("literllm_client_new: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Free a client created by [`literllm_client_new`].
///
/// # Safety
///
/// - `client` must be a valid pointer returned by `literllm_client_new`.
/// - `client` must not be used after this call (use-after-free is UB).
/// - Passing `NULL` is safe and is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_client_free(client: *mut LiterLlmClient) {
    // SAFETY: `client` is either NULL (safe to skip) or was returned by
    // `literllm_client_new`, which heap-allocates a `Box<LiterLlmClient>` via
    // `Box::into_raw`.  Reconstructing the `Box` here transfers ownership back
    // to Rust, which drops it at the end of this scope.
    if !client.is_null() {
        drop(unsafe { Box::from_raw(client) });
    }
}

/// Send a chat completion request.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `ChatCompletionRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ChatCompletionResponse` on success, or `NULL` on failure.
/// Check [`literllm_last_error`] on failure.
///
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_chat(client: *const LiterLlmClient, request_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_chat: client must not be NULL".into());
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error("literllm_chat: request_json must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literllm_chat: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literllm_chat: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_chat: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.chat(request));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_chat: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literllm_chat: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literllm_chat: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Callback invoked for each SSE chunk during a streaming chat completion.
///
/// - `chunk_json`: NUL-terminated JSON string for one `ChatCompletionChunk`.
///   The pointer is valid only for the duration of the callback invocation.
///   The callee must **not** free it.
/// - `user_data`: The opaque pointer passed to [`literllm_chat_stream`].
///
/// This callback returns void; there is no return value.
pub type LiterLlmStreamCallback = unsafe extern "C" fn(chunk_json: *const c_char, user_data: *mut std::ffi::c_void);

/// Send a streaming chat completion request, invoking a callback for each chunk.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `ChatCompletionRequest` schema.
/// - `callback`: Function called once per SSE chunk with the JSON-serialised
///   `ChatCompletionChunk`.  The `chunk_json` pointer is valid only for the
///   duration of each callback invocation and must **not** be freed.
/// - `user_data`: Opaque pointer forwarded unchanged to each `callback` call.
///   May be `NULL`.
///
/// # Return value
///
/// Returns `0` on success (all chunks delivered) or `-1` on failure.
/// Check [`literllm_last_error`] on failure.
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
/// - `callback` must be a valid function pointer; it is invoked from the calling
///   thread with the Tokio runtime blocked.
/// - `user_data` is forwarded as-is; the caller is responsible for its lifetime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_chat_stream(
    client: *const LiterLlmClient,
    request_json: *const c_char,
    callback: LiterLlmStreamCallback,
    user_data: *mut std::ffi::c_void,
) -> i32 {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_chat_stream: client must not be NULL".into());
        return -1;
    }
    if request_json.is_null() {
        set_last_error("literllm_chat_stream: request_json must not be NULL".into());
        return -1;
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literllm_chat_stream: request_json is not valid UTF-8: {e}"));
            return -1;
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literllm_chat_stream: failed to parse request JSON: {e}"));
            return -1;
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_chat_stream: {e}"));
            return -1;
        }
    };

    // Block on obtaining the stream, then iterate every chunk synchronously,
    // invoking the callback for each one.  C FFI callers cannot model async
    // iterators natively, so a blocking callback pattern is the correct API.
    let result = rt.block_on(async {
        use futures_core::Stream;
        use std::pin::Pin;

        let mut stream = match client_ref.chat_stream(request).await {
            Ok(s) => s,
            Err(e) => return Err(format!("literllm_chat_stream: failed to open stream: {e}")),
        };

        loop {
            let next = std::future::poll_fn(|cx| Pin::new(&mut stream).poll_next(cx)).await;
            match next {
                None => break,
                Some(Err(e)) => return Err(format!("literllm_chat_stream: stream error: {e}")),
                Some(Ok(chunk)) => {
                    let chunk_json = match serde_json::to_string(&chunk) {
                        Ok(s) => s,
                        Err(e) => return Err(format!("literllm_chat_stream: failed to serialise chunk: {e}")),
                    };
                    match CString::new(chunk_json) {
                        Ok(c_str) => {
                            // SAFETY: `callback` is a valid function pointer supplied
                            // by the caller.  `c_str.as_ptr()` is valid for this block
                            // scope and must not be stored or freed by the callee.
                            // `user_data` is forwarded as-is; ownership stays with the caller.
                            unsafe { callback(c_str.as_ptr(), user_data) };
                        }
                        Err(e) => return Err(format!("literllm_chat_stream: chunk JSON contained NUL byte: {e}")),
                    }
                }
            }
        }
        Ok(())
    });

    match result {
        Ok(()) => 0,
        Err(e) => {
            set_last_error(e);
            -1
        }
    }
}

/// Send an embedding request.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `EmbeddingRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `EmbeddingResponse` on success, or `NULL` on failure.
/// Check [`literllm_last_error`] on failure.
///
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_embed(client: *const LiterLlmClient, request_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_embed: client must not be NULL".into());
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error("literllm_embed: request_json must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literllm_embed: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literllm_embed: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_embed: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.embed(request));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_embed: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literllm_embed: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literllm_embed: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// List available models.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ModelsListResponse` on success, or `NULL` on failure.
/// Check [`literllm_last_error`] on failure.
///
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_list_models(client: *const LiterLlmClient) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_list_models: client must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` is non-null and was returned by
    // `literllm_client_new`.  The shared reference is valid for the duration
    // of this call.
    let client_ref = unsafe { &(*client).inner };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_list_models: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.list_models());

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_list_models: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literllm_list_models: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literllm_list_models: {e}"));
            std::ptr::null_mut()
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: JSON-in / JSON-out for request-based endpoints
// ---------------------------------------------------------------------------

/// Internal helper shared by all JSON-in/JSON-out FFI functions that take a
/// `(client, request_json)` pair.  Validates inputs, deserialises the JSON,
/// calls `op` inside the Tokio runtime, serialises the response, and returns
/// an owned `*mut c_char` (or `NULL` on error, with `LAST_ERROR` set).
///
/// `name` is used only for error messages.
fn json_request_response<Req, Resp>(
    name: &str,
    client: *const LiterLlmClient,
    request_json: *const c_char,
    op: impl for<'a> FnOnce(
        &'a DefaultClient,
        Req,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = liter_llm::error::Result<Resp>> + Send + 'a>,
    >,
) -> *mut c_char
where
    Req: serde::de::DeserializeOwned,
    Resp: serde::Serialize,
{
    clear_last_error();

    if client.is_null() {
        set_last_error(format!("{name}: client must not be NULL"));
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error(format!("{name}: request_json must not be NULL"));
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("{name}: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request: Req = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("{name}: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(op(client_ref, request));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("{name}: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("{name}: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Internal helper for endpoints that take `(client, id_string)` and return JSON.
fn id_request_response<Resp>(
    name: &str,
    client: *const LiterLlmClient,
    id_ptr: *const c_char,
    id_label: &str,
    op: impl for<'a> FnOnce(
        &'a DefaultClient,
        &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = liter_llm::error::Result<Resp>> + Send + 'a>,
    >,
) -> *mut c_char
where
    Resp: serde::Serialize,
{
    clear_last_error();

    if client.is_null() {
        set_last_error(format!("{name}: client must not be NULL"));
        return std::ptr::null_mut();
    }
    if id_ptr.is_null() {
        set_last_error(format!("{name}: {id_label} must not be NULL"));
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `id_ptr` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let id_str = match unsafe { CStr::from_ptr(id_ptr) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("{name}: {id_label} is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(op(client_ref, id_str));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("{name}: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("{name}: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Internal helper for endpoints that return raw bytes (encoded as base64 JSON).
fn id_request_bytes(
    name: &str,
    client: *const LiterLlmClient,
    id_ptr: *const c_char,
    id_label: &str,
    op: impl for<'a> FnOnce(
        &'a DefaultClient,
        &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = liter_llm::error::Result<bytes::Bytes>> + Send + 'a>,
    >,
) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error(format!("{name}: client must not be NULL"));
        return std::ptr::null_mut();
    }
    if id_ptr.is_null() {
        set_last_error(format!("{name}: {id_label} must not be NULL"));
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `id_ptr` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let id_str = match unsafe { CStr::from_ptr(id_ptr) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("{name}: {id_label} is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(op(client_ref, id_str));

    match result {
        Ok(data) => {
            use base64::Engine;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
            match CString::new(encoded) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("{name}: base64 output contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            }
        }
        Err(e) => {
            set_last_error(format!("{name}: {e}"));
            std::ptr::null_mut()
        }
    }
}

// ---------------------------------------------------------------------------
// Inference API: image_generate, speech, transcribe, moderate, rerank
// ---------------------------------------------------------------------------

/// Generate an image from a text prompt.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateImageRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ImagesResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_image_generate(
    client: *const LiterLlmClient,
    request_json: *const c_char,
) -> *mut c_char {
    json_request_response("literllm_image_generate", client, request_json, |c, req| {
        Box::pin(c.image_generate(req))
    })
}

/// Generate speech audio from text.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateSpeechRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated base64-encoded string of the audio
/// bytes on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_speech(client: *const LiterLlmClient, request_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_speech: client must not be NULL".into());
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error("literllm_speech: request_json must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees both pointers are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literllm_speech: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literllm_speech: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_speech: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.speech(request));

    match result {
        Ok(data) => {
            use base64::Engine;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
            match CString::new(encoded) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_speech: base64 output contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            }
        }
        Err(e) => {
            set_last_error(format!("literllm_speech: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Transcribe audio to text.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateTranscriptionRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `TranscriptionResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_transcribe(
    client: *const LiterLlmClient,
    request_json: *const c_char,
) -> *mut c_char {
    json_request_response("literllm_transcribe", client, request_json, |c, req| {
        Box::pin(c.transcribe(req))
    })
}

/// Check content against moderation policies.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `ModerationRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ModerationResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_moderate(client: *const LiterLlmClient, request_json: *const c_char) -> *mut c_char {
    json_request_response("literllm_moderate", client, request_json, |c, req| {
        Box::pin(c.moderate(req))
    })
}

/// Rerank documents by relevance to a query.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `RerankRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `RerankResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_rerank(client: *const LiterLlmClient, request_json: *const c_char) -> *mut c_char {
    json_request_response("literllm_rerank", client, request_json, |c, req| {
        Box::pin(c.rerank(req))
    })
}

// ---------------------------------------------------------------------------
// File management API
// ---------------------------------------------------------------------------

/// Upload a file.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateFileRequest` schema.  The `file` field must be base64-encoded.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `FileObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_create_file(
    client: *const LiterLlmClient,
    request_json: *const c_char,
) -> *mut c_char {
    json_request_response("literllm_create_file", client, request_json, |c, req| {
        Box::pin(c.create_file(req))
    })
}

/// Retrieve metadata for a file by ID.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `file_id`: NUL-terminated file ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `FileObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `file_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_retrieve_file(client: *const LiterLlmClient, file_id: *const c_char) -> *mut c_char {
    id_request_response("literllm_retrieve_file", client, file_id, "file_id", |c, id| {
        Box::pin(c.retrieve_file(id))
    })
}

/// Delete a file by ID.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `file_id`: NUL-terminated file ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `DeleteResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `file_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_delete_file(client: *const LiterLlmClient, file_id: *const c_char) -> *mut c_char {
    id_request_response("literllm_delete_file", client, file_id, "file_id", |c, id| {
        Box::pin(c.delete_file(id))
    })
}

/// List files, optionally filtered by query parameters.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `query_json`: NUL-terminated JSON string conforming to the
///   `FileListQuery` schema.  May be `NULL` to list all files.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `FileListResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `query_json` may be `NULL` or a valid NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_list_files(client: *const LiterLlmClient, query_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_list_files: client must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` is non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let query: Option<liter_llm::types::files::FileListQuery> = if query_json.is_null() {
        None
    } else {
        // SAFETY: caller guarantees `query_json` is a valid NUL-terminated string.
        let json_str = match unsafe { CStr::from_ptr(query_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("literllm_list_files: query_json is not valid UTF-8: {e}"));
                return std::ptr::null_mut();
            }
        };
        match serde_json::from_str(json_str) {
            Ok(q) => Some(q),
            Err(e) => {
                set_last_error(format!("literllm_list_files: failed to parse query JSON: {e}"));
                return std::ptr::null_mut();
            }
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_list_files: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.list_files(query));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_list_files: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literllm_list_files: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literllm_list_files: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Retrieve the raw content of a file (returned as base64-encoded string).
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `file_id`: NUL-terminated file ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated base64-encoded string of the file
/// content on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `file_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_file_content(client: *const LiterLlmClient, file_id: *const c_char) -> *mut c_char {
    id_request_bytes("literllm_file_content", client, file_id, "file_id", |c, id| {
        Box::pin(c.file_content(id))
    })
}

// ---------------------------------------------------------------------------
// Batch API
// ---------------------------------------------------------------------------

/// Create a new batch job.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateBatchRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `BatchObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_create_batch(
    client: *const LiterLlmClient,
    request_json: *const c_char,
) -> *mut c_char {
    json_request_response("literllm_create_batch", client, request_json, |c, req| {
        Box::pin(c.create_batch(req))
    })
}

/// Retrieve a batch by ID.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `batch_id`: NUL-terminated batch ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `BatchObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `batch_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_retrieve_batch(
    client: *const LiterLlmClient,
    batch_id: *const c_char,
) -> *mut c_char {
    id_request_response("literllm_retrieve_batch", client, batch_id, "batch_id", |c, id| {
        Box::pin(c.retrieve_batch(id))
    })
}

/// List batches, optionally filtered by query parameters.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `query_json`: NUL-terminated JSON string conforming to the
///   `BatchListQuery` schema.  May be `NULL` to list all batches.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `BatchListResponse` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `query_json` may be `NULL` or a valid NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_list_batches(
    client: *const LiterLlmClient,
    query_json: *const c_char,
) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literllm_list_batches: client must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` is non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let query: Option<liter_llm::types::batch::BatchListQuery> = if query_json.is_null() {
        None
    } else {
        // SAFETY: caller guarantees `query_json` is a valid NUL-terminated string.
        let json_str = match unsafe { CStr::from_ptr(query_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("literllm_list_batches: query_json is not valid UTF-8: {e}"));
                return std::ptr::null_mut();
            }
        };
        match serde_json::from_str(json_str) {
            Ok(q) => Some(q),
            Err(e) => {
                set_last_error(format!("literllm_list_batches: failed to parse query JSON: {e}"));
                return std::ptr::null_mut();
            }
        }
    };

    let rt = match runtime() {
        Ok(rt) => rt,
        Err(e) => {
            set_last_error(format!("literllm_list_batches: {e}"));
            return std::ptr::null_mut();
        }
    };
    let result = rt.block_on(client_ref.list_batches(query));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literllm_list_batches: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literllm_list_batches: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literllm_list_batches: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Cancel an in-progress batch.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `batch_id`: NUL-terminated batch ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `BatchObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `batch_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_cancel_batch(client: *const LiterLlmClient, batch_id: *const c_char) -> *mut c_char {
    id_request_response("literllm_cancel_batch", client, batch_id, "batch_id", |c, id| {
        Box::pin(c.cancel_batch(id))
    })
}

// ---------------------------------------------------------------------------
// Responses API
// ---------------------------------------------------------------------------

/// Create a new response.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `request_json`: NUL-terminated JSON string conforming to the
///   `CreateResponseRequest` schema.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ResponseObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_create_response(
    client: *const LiterLlmClient,
    request_json: *const c_char,
) -> *mut c_char {
    json_request_response("literllm_create_response", client, request_json, |c, req| {
        Box::pin(c.create_response(req))
    })
}

/// Retrieve a response by ID.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `response_id`: NUL-terminated response ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ResponseObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `response_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_retrieve_response(
    client: *const LiterLlmClient,
    response_id: *const c_char,
) -> *mut c_char {
    id_request_response(
        "literllm_retrieve_response",
        client,
        response_id,
        "response_id",
        |c, id| Box::pin(c.retrieve_response(id)),
    )
}

/// Cancel an in-progress response.
///
/// # Parameters
///
/// - `client`: A valid client pointer.
/// - `response_id`: NUL-terminated response ID string.
///
/// # Return value
///
/// Returns a heap-allocated NUL-terminated JSON string containing the
/// `ResponseObject` on success, or `NULL` on failure.
/// The caller must free the returned string with [`literllm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literllm_client_new`.
/// - `response_id` must be a valid, non-null, NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_cancel_response(
    client: *const LiterLlmClient,
    response_id: *const c_char,
) -> *mut c_char {
    id_request_response(
        "literllm_cancel_response",
        client,
        response_id,
        "response_id",
        |c, id| Box::pin(c.cancel_response(id)),
    )
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/// Retrieve the last error message for the current thread.
///
/// Returns a `const char*` pointer to the NUL-terminated error string, or
/// `NULL` if no error has occurred since the last successful call.
///
/// The returned pointer is valid only until the **next** liter-llm function
/// call on the **same thread**.  The caller must **not** free this pointer.
///
/// # Safety
///
/// Always safe to call.  No preconditions.
#[unsafe(no_mangle)]
pub extern "C" fn literllm_last_error() -> *const c_char {
    LAST_ERROR.with(|cell| match &*cell.borrow() {
        Some(c_str) => c_str.as_ptr(),
        None => std::ptr::null(),
    })
}

/// Free a string returned by [`literllm_chat`], [`literllm_embed`], or
/// [`literllm_list_models`].
///
/// # Safety
///
/// - `s` must be a pointer returned by one of the functions listed above.
/// - `s` must not be used after this call (use-after-free is UB).
/// - Passing `NULL` is safe and is a no-op.
/// - Do **not** pass the pointer returned by [`literllm_last_error`]; that
///   pointer must not be freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literllm_free_string(s: *mut c_char) {
    // SAFETY: `s` is either NULL (no-op) or was returned by `CString::into_raw`
    // inside this crate.  Reconstructing the `CString` transfers ownership back
    // to Rust, which drops and deallocates the allocation at end of scope.
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

/// Returns the version string of the liter-llm library.
///
/// The returned pointer is valid for the lifetime of the program and must
/// **not** be freed.
///
/// # Safety
///
/// Always safe to call.
#[unsafe(no_mangle)]
pub extern "C" fn literllm_version() -> *const c_char {
    // SAFETY: VERSION is 'static, NUL-terminated, and lives for the duration
    // of the program.  It is initialised exactly once via OnceLock on first
    // call.  The raw pointer is never freed by the caller (documented above).
    //
    // `CARGO_PKG_VERSION` is set by Cargo at compile time and never contains
    // interior NUL bytes (semver syntax does not include NUL).
    static VERSION: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
    VERSION
        .get_or_init(|| {
            // SAFETY: semver strings (e.g. "1.0.0") never contain NUL bytes,
            // so `CString::new` will always succeed here.
            CString::new(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| c"unknown".to_owned())
        })
        .as_ptr()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn version_is_non_null() {
        let ptr = literllm_version();
        assert!(!ptr.is_null());
        // SAFETY: `ptr` points to a static NUL-terminated string.
        let s = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        assert!(s.contains('.'), "version should contain a dot: {s}");
    }

    #[test]
    fn last_error_null_initially() {
        clear_last_error();
        let ptr = literllm_last_error();
        assert!(ptr.is_null(), "last error should be null when none set");
    }

    #[test]
    fn last_error_returns_message_after_set() {
        set_last_error("something went wrong".into());
        let ptr = literllm_last_error();
        assert!(!ptr.is_null());
        // SAFETY: `ptr` is valid until the next liter-llm call on this thread.
        let msg = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        assert_eq!(msg, "something went wrong");
        clear_last_error();
    }

    #[test]
    fn client_new_null_api_key_returns_null() {
        // SAFETY: passing NULL api_key is documented to return NULL + set error.
        let client = unsafe { literllm_client_new(std::ptr::null(), std::ptr::null(), std::ptr::null()) };
        assert!(client.is_null());
        let err = literllm_last_error();
        assert!(!err.is_null());
        // SAFETY: err is valid until next call on this thread.
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("NULL"));
    }

    #[test]
    fn client_new_and_free_empty_key() {
        let api_key = CString::new("test-key").unwrap();
        // SAFETY: api_key is a valid NUL-terminated string; base_url and model_hint are NULL.
        let client = unsafe { literllm_client_new(api_key.as_ptr(), std::ptr::null(), std::ptr::null()) };
        // Construction may fail if reqwest internals fail, but on CI it should succeed.
        if !client.is_null() {
            // SAFETY: client was returned by literllm_client_new.
            unsafe { literllm_client_free(client) };
        }
    }

    #[test]
    fn client_free_null_is_noop() {
        // SAFETY: NULL is documented to be safe.
        unsafe { literllm_client_free(std::ptr::null_mut()) };
    }

    #[test]
    fn free_string_null_is_noop() {
        // SAFETY: NULL is documented to be safe.
        unsafe { literllm_free_string(std::ptr::null_mut()) };
    }

    #[test]
    fn chat_null_client_returns_null() {
        let req = CString::new("{}").unwrap();
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literllm_chat(std::ptr::null(), req.as_ptr()) };
        assert!(result.is_null());
        let err = literllm_last_error();
        assert!(!err.is_null());
    }

    #[test]
    fn chat_null_request_returns_null() {
        let api_key = CString::new("test-key").unwrap();
        // SAFETY: api_key is valid; base_url and model_hint are NULL.
        let client = unsafe { literllm_client_new(api_key.as_ptr(), std::ptr::null(), std::ptr::null()) };
        if client.is_null() {
            return; // skip if construction failed
        }
        // SAFETY: client is valid; request_json is NULL (should return NULL + error).
        let result = unsafe { literllm_chat(client, std::ptr::null()) };
        assert!(result.is_null());
        let err = literllm_last_error();
        assert!(!err.is_null());
        // SAFETY: client was returned by literllm_client_new.
        unsafe { literllm_client_free(client) };
    }

    #[test]
    fn embed_null_client_returns_null() {
        let req = CString::new("{}").unwrap();
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literllm_embed(std::ptr::null(), req.as_ptr()) };
        assert!(result.is_null());
    }

    #[test]
    fn list_models_null_client_returns_null() {
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literllm_list_models(std::ptr::null()) };
        assert!(result.is_null());
        let err = literllm_last_error();
        assert!(!err.is_null());
    }
}
