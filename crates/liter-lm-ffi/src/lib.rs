//! C FFI bindings for liter-lm.
//!
//! Provides an opaque-handle C API consumed by Go (cgo), Java (Panama FFM),
//! C# (P/Invoke), and any other language with C FFI support.
//!
//! ## Ownership model
//!
//! - [`literlm_client_new`] returns a heap-allocated `*mut LiterLmClient`.
//!   The caller **owns** it and must eventually call [`literlm_client_free`].
//! - [`literlm_chat`], [`literlm_embed`], [`literlm_list_models`] return
//!   heap-allocated `*mut c_char` JSON strings.
//!   The caller **owns** them and must call [`literlm_free_string`].
//! - [`literlm_last_error`] returns a thread-local `*const c_char`.
//!   The caller must **not** free it; it is valid until the next call on the
//!   same thread.

use std::ffi::{CStr, CString, c_char};

use liter_lm::client::{ClientConfig, DefaultClient, LlmClient};

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

/// Opaque handle to a liter-lm client.
///
/// Create with [`literlm_client_new`], destroy with [`literlm_client_free`].
/// All fields are private; callers interact only through the public functions.
///
/// cbindgen:no-export — we emit the opaque declaration manually in the header
/// preamble so C callers only ever hold a `LiterLmClient*`.
pub struct LiterLmClient {
    inner: DefaultClient,
}

/// Tokio runtime used for blocking on async operations from synchronous C callers.
///
/// A single runtime is created on first use and shared across all threads.
fn runtime() -> &'static tokio::runtime::Runtime {
    static RT: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    })
}

// ---------------------------------------------------------------------------
// Public C API
// ---------------------------------------------------------------------------

/// Create a new liter-lm client.
///
/// # Parameters
///
/// - `api_key`: NUL-terminated API key string.  Pass an empty string (`""`)
///   when using a provider that does not require authentication.
/// - `base_url`: NUL-terminated base URL override.  Pass `NULL` to use the
///   default provider routing based on model-name prefix.
///
/// # Return value
///
/// Returns a heap-allocated `LiterLmClient*` on success, or `NULL` on failure.
/// Check [`literlm_last_error`] for the error message when `NULL` is returned.
///
/// The returned pointer must be freed with [`literlm_client_free`].
///
/// # Safety
///
/// - `api_key` must be a valid, non-null, NUL-terminated C string.
/// - `base_url` may be `NULL` (treated as no override) or a valid NUL-terminated C string.
/// - The caller owns the returned pointer and must call `literlm_client_free` exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_client_new(api_key: *const c_char, base_url: *const c_char) -> *mut LiterLmClient {
    clear_last_error();

    // SAFETY: caller guarantees `api_key` is non-null and NUL-terminated.
    if api_key.is_null() {
        set_last_error("literlm_client_new: api_key must not be NULL".into());
        return std::ptr::null_mut();
    }

    let api_key_str = match unsafe { CStr::from_ptr(api_key) }.to_str() {
        Ok(s) => s.to_owned(),
        Err(e) => {
            set_last_error(format!("literlm_client_new: api_key is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let mut config_builder = liter_lm::client::ClientConfigBuilder::new(api_key_str);

    // SAFETY: `base_url` is either NULL (skip) or a valid NUL-terminated C string.
    if !base_url.is_null() {
        match unsafe { CStr::from_ptr(base_url) }.to_str() {
            Ok(url) if !url.is_empty() => {
                config_builder = config_builder.base_url(url);
            }
            Ok(_) => {} // empty string — treat as no override
            Err(e) => {
                set_last_error(format!("literlm_client_new: base_url is not valid UTF-8: {e}"));
                return std::ptr::null_mut();
            }
        }
    }

    let config: ClientConfig = config_builder.build();

    match DefaultClient::new(config, None) {
        Ok(client) => {
            let handle = Box::new(LiterLmClient { inner: client });
            Box::into_raw(handle)
        }
        Err(e) => {
            set_last_error(format!("literlm_client_new: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Free a client created by [`literlm_client_new`].
///
/// # Safety
///
/// - `client` must be a valid pointer returned by `literlm_client_new`.
/// - `client` must not be used after this call (use-after-free is UB).
/// - Passing `NULL` is safe and is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_client_free(client: *mut LiterLmClient) {
    // SAFETY: `client` is either NULL (safe to skip) or was returned by
    // `literlm_client_new`, which heap-allocates a `Box<LiterLmClient>` via
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
/// Check [`literlm_last_error`] on failure.
///
/// The caller must free the returned string with [`literlm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literlm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_chat(client: *const LiterLmClient, request_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literlm_chat: client must not be NULL".into());
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error("literlm_chat: request_json must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literlm_chat: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literlm_chat: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let result = runtime().block_on(client_ref.chat(request));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literlm_chat: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literlm_chat: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literlm_chat: {e}"));
            std::ptr::null_mut()
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
/// Check [`literlm_last_error`] on failure.
///
/// The caller must free the returned string with [`literlm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literlm_client_new`.
/// - `request_json` must be a valid, non-null, NUL-terminated UTF-8 JSON string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_embed(client: *const LiterLmClient, request_json: *const c_char) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literlm_embed: client must not be NULL".into());
        return std::ptr::null_mut();
    }
    if request_json.is_null() {
        set_last_error("literlm_embed: request_json must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` and `request_json` are non-null and valid.
    let client_ref = unsafe { &(*client).inner };

    let json_str = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(format!("literlm_embed: request_json is not valid UTF-8: {e}"));
            return std::ptr::null_mut();
        }
    };

    let request = match serde_json::from_str(json_str) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(format!("literlm_embed: failed to parse request JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let result = runtime().block_on(client_ref.embed(request));

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literlm_embed: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literlm_embed: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literlm_embed: {e}"));
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
/// Check [`literlm_last_error`] on failure.
///
/// The caller must free the returned string with [`literlm_free_string`].
///
/// # Safety
///
/// - `client` must be a valid, non-null pointer returned by `literlm_client_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_list_models(client: *const LiterLmClient) -> *mut c_char {
    clear_last_error();

    if client.is_null() {
        set_last_error("literlm_list_models: client must not be NULL".into());
        return std::ptr::null_mut();
    }

    // SAFETY: caller guarantees `client` is non-null and was returned by
    // `literlm_client_new`.  The shared reference is valid for the duration
    // of this call.
    let client_ref = unsafe { &(*client).inner };

    let result = runtime().block_on(client_ref.list_models());

    match result {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(json) => match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(e) => {
                    set_last_error(format!("literlm_list_models: response JSON contained NUL byte: {e}"));
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(format!("literlm_list_models: failed to serialize response: {e}"));
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(format!("literlm_list_models: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// Retrieve the last error message for the current thread.
///
/// Returns a `const char*` pointer to the NUL-terminated error string, or
/// `NULL` if no error has occurred since the last successful call.
///
/// The returned pointer is valid only until the **next** liter-lm function
/// call on the **same thread**.  The caller must **not** free this pointer.
///
/// # Safety
///
/// Always safe to call.  No preconditions.
#[unsafe(no_mangle)]
pub extern "C" fn literlm_last_error() -> *const c_char {
    LAST_ERROR.with(|cell| match &*cell.borrow() {
        Some(c_str) => c_str.as_ptr(),
        None => std::ptr::null(),
    })
}

/// Free a string returned by [`literlm_chat`], [`literlm_embed`], or
/// [`literlm_list_models`].
///
/// # Safety
///
/// - `s` must be a pointer returned by one of the functions listed above.
/// - `s` must not be used after this call (use-after-free is UB).
/// - Passing `NULL` is safe and is a no-op.
/// - Do **not** pass the pointer returned by [`literlm_last_error`]; that
///   pointer must not be freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn literlm_free_string(s: *mut c_char) {
    // SAFETY: `s` is either NULL (no-op) or was returned by `CString::into_raw`
    // inside this crate.  Reconstructing the `CString` transfers ownership back
    // to Rust, which drops and deallocates the allocation at end of scope.
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

/// Returns the version string of the liter-lm library.
///
/// The returned pointer is valid for the lifetime of the program and must
/// **not** be freed.
///
/// # Safety
///
/// Always safe to call.
#[unsafe(no_mangle)]
pub extern "C" fn liter_lm_version() -> *const c_char {
    // SAFETY: This string literal is 'static and NUL-terminated.
    c"0.1.0".as_ptr()
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
        let ptr = liter_lm_version();
        assert!(!ptr.is_null());
        // SAFETY: `ptr` points to a static NUL-terminated string.
        let s = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        assert!(s.contains('.'), "version should contain a dot: {s}");
    }

    #[test]
    fn last_error_null_initially() {
        clear_last_error();
        let ptr = literlm_last_error();
        assert!(ptr.is_null(), "last error should be null when none set");
    }

    #[test]
    fn last_error_returns_message_after_set() {
        set_last_error("something went wrong".into());
        let ptr = literlm_last_error();
        assert!(!ptr.is_null());
        // SAFETY: `ptr` is valid until the next liter-lm call on this thread.
        let msg = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        assert_eq!(msg, "something went wrong");
        clear_last_error();
    }

    #[test]
    fn client_new_null_api_key_returns_null() {
        // SAFETY: passing NULL api_key is documented to return NULL + set error.
        let client = unsafe { literlm_client_new(std::ptr::null(), std::ptr::null()) };
        assert!(client.is_null());
        let err = literlm_last_error();
        assert!(!err.is_null());
        // SAFETY: err is valid until next call on this thread.
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("NULL"));
    }

    #[test]
    fn client_new_and_free_empty_key() {
        let api_key = CString::new("test-key").unwrap();
        // SAFETY: api_key is a valid NUL-terminated string; base_url is NULL.
        let client = unsafe { literlm_client_new(api_key.as_ptr(), std::ptr::null()) };
        // Construction may fail if reqwest internals fail, but on CI it should succeed.
        if !client.is_null() {
            // SAFETY: client was returned by literlm_client_new.
            unsafe { literlm_client_free(client) };
        }
    }

    #[test]
    fn client_free_null_is_noop() {
        // SAFETY: NULL is documented to be safe.
        unsafe { literlm_client_free(std::ptr::null_mut()) };
    }

    #[test]
    fn free_string_null_is_noop() {
        // SAFETY: NULL is documented to be safe.
        unsafe { literlm_free_string(std::ptr::null_mut()) };
    }

    #[test]
    fn chat_null_client_returns_null() {
        let req = CString::new("{}").unwrap();
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literlm_chat(std::ptr::null(), req.as_ptr()) };
        assert!(result.is_null());
        let err = literlm_last_error();
        assert!(!err.is_null());
    }

    #[test]
    fn chat_null_request_returns_null() {
        let api_key = CString::new("test-key").unwrap();
        // SAFETY: api_key is valid; base_url is NULL.
        let client = unsafe { literlm_client_new(api_key.as_ptr(), std::ptr::null()) };
        if client.is_null() {
            return; // skip if construction failed
        }
        // SAFETY: client is valid; request_json is NULL (should return NULL + error).
        let result = unsafe { literlm_chat(client, std::ptr::null()) };
        assert!(result.is_null());
        let err = literlm_last_error();
        assert!(!err.is_null());
        // SAFETY: client was returned by literlm_client_new.
        unsafe { literlm_client_free(client) };
    }

    #[test]
    fn embed_null_client_returns_null() {
        let req = CString::new("{}").unwrap();
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literlm_embed(std::ptr::null(), req.as_ptr()) };
        assert!(result.is_null());
    }

    #[test]
    fn list_models_null_client_returns_null() {
        // SAFETY: NULL client is documented to return NULL + set error.
        let result = unsafe { literlm_list_models(std::ptr::null()) };
        assert!(result.is_null());
        let err = literlm_last_error();
        assert!(!err.is_null());
    }
}
