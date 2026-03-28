use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use liter_llm::tower::{BudgetConfig, CacheConfig, Enforcement, RateLimitConfig};
use liter_llm::tower::{LlmHook, LlmRequest, LlmResponse};
use liter_llm::{
    AuthHeaderFormat, BatchClient, ClientConfigBuilder, CustomProviderConfig, FileClient, LiterLlmError, LlmClient,
    ManagedClient, ResponseClient, register_custom_provider, unregister_custom_provider,
};
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tokio::sync::{Mutex, RwLock, mpsc};
use tokio::task::JoinHandle;

use crate::error::to_py_err;
use crate::types::{
    PyChatCompletionChunk, PyChatCompletionResponse, PyEmbeddingResponse, PyImagesResponse, PyModelsListResponse,
    PyModerationResponse, PyRerankResponse, PyTranscriptionResponse, to_json_value,
};

// ─── Python Hook Bridge ──────────────────────────────────────────────────────

/// A bridge that implements `LlmHook` by calling back into Python objects.
///
/// The Python hook object may define any combination of:
///   - `on_request(request: dict) -> None`   (may be async)
///   - `on_response(request: dict, response: dict) -> None`  (may be async)
///   - `on_error(request: dict, error: str) -> None`  (may be async)
///
/// Missing methods are silently ignored (no-op).
struct PyHookBridge {
    /// The Python hook object stored as an `Arc<Py<PyAny>>` so it can be
    /// cheaply cloned without holding the GIL and shared into `spawn_blocking`
    /// closures that require `'static`.
    hook: Arc<Py<PyAny>>,
}

// SAFETY: `Py<PyAny>` is `Send + Sync` by design — it holds a reference-counted
// pointer to a Python object and only accesses it while the GIL is held.
unsafe impl Send for PyHookBridge {}
unsafe impl Sync for PyHookBridge {}

impl PyHookBridge {
    fn new(hook: Py<PyAny>) -> Self {
        Self { hook: Arc::new(hook) }
    }

    /// Call a named method on the Python hook object.
    ///
    /// Runs inside `spawn_blocking` + `Python::attach` so the GIL is acquired
    /// on a blocking thread and the Tokio runtime is never blocked.
    ///
    /// If the Python method returns an awaitable the bridge uses `asyncio.run`
    /// to drive it (safe because we are on a dedicated blocking thread).
    fn call_method_fire_and_forget(
        &self,
        method_name: &'static str,
        args: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let hook = Arc::clone(&self.hook);
        Box::pin(async move {
            let _ = tokio::task::spawn_blocking(move || {
                if let Some(py) = Python::try_attach(|py| {
                    let obj = hook.bind(py);
                    let method = match obj.getattr(method_name) {
                        Ok(m) => m,
                        Err(_) => return, // method not defined — no-op
                    };
                    let py_args = pyo3::types::PyTuple::new(py, args.iter().map(|s| s.as_str())).expect("tuple");
                    let result = match method.call1(py_args) {
                        Ok(r) => r,
                        Err(_) => return,
                    };
                    // Drive coroutines synchronously on this blocking thread.
                    if result.hasattr("__await__").unwrap_or(false)
                        && let Ok(asyncio) = py.import("asyncio")
                    {
                        let _ = asyncio.call_method1("run", (result,));
                    }
                }) {
                    py
                }
            })
            .await;
        })
    }

    /// Like `call_method_fire_and_forget` but returns `Result` so `on_request`
    /// can reject the request by raising a Python exception.
    fn call_method_checked(
        &self,
        method_name: &'static str,
        args: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<(), LiterLlmError>> + Send + '_>> {
        let hook = Arc::clone(&self.hook);
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                Python::try_attach(|py| {
                    let obj = hook.bind(py);
                    let method = match obj.getattr(method_name) {
                        Ok(m) => m,
                        Err(_) => return Ok(()), // method not defined — no-op
                    };
                    let py_args = pyo3::types::PyTuple::new(py, args.iter().map(|s| s.as_str())).map_err(|e| {
                        LiterLlmError::HookRejected {
                            message: format!("failed to build hook arguments: {e}"),
                        }
                    })?;
                    let result = method.call1(py_args).map_err(|e| LiterLlmError::HookRejected {
                        message: format!("hook {method_name} raised: {e}"),
                    })?;
                    // Drive coroutines synchronously on this blocking thread.
                    if result.hasattr("__await__").unwrap_or(false) {
                        let asyncio = py.import("asyncio").map_err(|e| LiterLlmError::HookRejected {
                            message: format!("failed to import asyncio: {e}"),
                        })?;
                        asyncio
                            .call_method1("run", (result,))
                            .map_err(|e| LiterLlmError::HookRejected {
                                message: format!("hook coroutine {method_name} raised: {e}"),
                            })?;
                    }
                    Ok(())
                })
                .unwrap_or(Err(LiterLlmError::HookRejected {
                    message: "failed to acquire Python GIL".into(),
                }))
            })
            .await
            .map_err(|e| LiterLlmError::HookRejected {
                message: format!("hook task panicked: {e}"),
            })?
        })
    }
}

/// Serialize the inner request of an [`LlmRequest`] to JSON.
fn request_to_json(req: &LlmRequest) -> String {
    match req {
        LlmRequest::Chat(r) | LlmRequest::ChatStream(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::Embed(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::ImageGenerate(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::Speech(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::Transcribe(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::Moderate(r) => serde_json::to_string(r).unwrap_or_default(),
        LlmRequest::Rerank(r) => serde_json::to_string(r).unwrap_or_default(),
        _ => format!("{req:?}"),
    }
}

impl LlmHook for PyHookBridge {
    fn on_request(&self, req: &LlmRequest) -> Pin<Box<dyn Future<Output = liter_llm::Result<()>> + Send + '_>> {
        let req_json = request_to_json(req);
        self.call_method_checked("on_request", vec![req_json])
    }

    fn on_response(&self, req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let req_json = request_to_json(req);
        self.call_method_fire_and_forget("on_response", vec![req_json, "response".to_owned()])
    }

    fn on_error(&self, req: &LlmRequest, err: &LiterLlmError) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let req_json = request_to_json(req);
        let err_msg = err.to_string();
        self.call_method_fire_and_forget("on_error", vec![req_json, err_msg])
    }
}

/// Parse a Python dict into a `CacheConfig`.
fn parse_cache_config(dict: &Bound<'_, PyDict>) -> PyResult<CacheConfig> {
    let max_entries: usize = dict
        .get_item("max_entries")?
        .map(|v| v.extract::<usize>())
        .transpose()?
        .unwrap_or(256);
    let ttl_seconds: u64 = dict
        .get_item("ttl_seconds")?
        .map(|v| v.extract::<u64>())
        .transpose()?
        .unwrap_or(300);
    Ok(CacheConfig {
        max_entries,
        ttl: std::time::Duration::from_secs(ttl_seconds),
        backend: Default::default(),
    })
}

/// Parse a Python dict into a `BudgetConfig`.
fn parse_budget_config(dict: &Bound<'_, PyDict>) -> PyResult<BudgetConfig> {
    let global_limit: Option<f64> = dict.get_item("global_limit")?.map(|v| v.extract::<f64>()).transpose()?;
    let model_limits: HashMap<String, f64> = dict
        .get_item("model_limits")?
        .map(|v| v.extract::<HashMap<String, f64>>())
        .transpose()?
        .unwrap_or_default();
    let enforcement_str: String = dict
        .get_item("enforcement")?
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "hard".to_owned());
    let enforcement = match enforcement_str.as_str() {
        "soft" => Enforcement::Soft,
        _ => Enforcement::Hard,
    };
    Ok(BudgetConfig {
        global_limit,
        model_limits,
        enforcement,
    })
}

/// Parse a Python dict into a `CustomProviderConfig`.
fn parse_provider_config(dict: &Bound<'_, PyDict>) -> PyResult<CustomProviderConfig> {
    let name: String = dict
        .get_item("name")?
        .ok_or_else(|| PyValueError::new_err("provider config requires 'name'"))?
        .extract()?;
    let base_url: String = dict
        .get_item("base_url")?
        .ok_or_else(|| PyValueError::new_err("provider config requires 'base_url'"))?
        .extract()?;
    let auth_header_str: String = dict
        .get_item("auth_header")?
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "bearer".to_owned());
    let auth_header = match auth_header_str.as_str() {
        "none" => AuthHeaderFormat::None,
        s if s.starts_with("api-key:") => AuthHeaderFormat::ApiKey(s.trim_start_matches("api-key:").trim().to_owned()),
        _ => AuthHeaderFormat::Bearer,
    };
    let model_prefixes: Vec<String> = dict
        .get_item("model_prefixes")?
        .map(|v| v.extract::<Vec<String>>())
        .transpose()?
        .unwrap_or_default();
    Ok(CustomProviderConfig {
        name,
        base_url,
        auth_header,
        model_prefixes,
    })
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Convert a Python dict (kwargs) into a `serde_json::Value` without importing
/// the Python `json` module.  This avoids holding the GIL for a round-trip
/// through Python's json.dumps and is safe across all JSON-serialisable Python
/// types (nested dicts, lists, scalars, None).
fn py_to_json(ob: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    // None → null
    if ob.is_none() {
        return Ok(serde_json::Value::Null);
    }
    // bool must be checked before int because Python bool is a subclass of int.
    if let Ok(b) = ob.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Ok(i) = ob.extract::<i64>() {
        return Ok(serde_json::Value::Number(i.into()));
    }
    if let Ok(f) = ob.extract::<f64>() {
        let n = serde_json::Number::from_f64(f)
            .ok_or_else(|| PyValueError::new_err(format!("non-finite float {f} cannot be serialised to JSON")))?;
        return Ok(serde_json::Value::Number(n));
    }
    if let Ok(s) = ob.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    // dict → object
    if let Ok(d) = ob.cast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in d.iter() {
            let key: String = k
                .extract()
                .map_err(|_| PyValueError::new_err("dict keys must be strings for JSON serialisation"))?;
            map.insert(key, py_to_json(&v)?);
        }
        return Ok(serde_json::Value::Object(map));
    }
    // list / tuple → array
    if let Ok(list) = ob.cast::<PyList>() {
        let items: PyResult<Vec<_>> = list.iter().map(|item| py_to_json(&item)).collect();
        return Ok(serde_json::Value::Array(items?));
    }
    if let Ok(seq) = ob.try_iter() {
        let items: PyResult<Vec<_>> = seq.map(|item| py_to_json(&item?)).collect();
        return Ok(serde_json::Value::Array(items?));
    }
    Err(PyValueError::new_err(format!(
        "cannot serialise object of type {} to JSON",
        ob.get_type()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "<unknown>".to_owned())
    )))
}

fn kwargs_to_json(kwargs: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (k, v) in kwargs.iter() {
        let key: String = k
            .extract()
            .map_err(|_| PyValueError::new_err("keyword argument names must be strings"))?;
        map.insert(key, py_to_json(&v)?);
    }
    Ok(serde_json::Value::Object(map))
}

// ─── LlmClient Python class ───────────────────────────────────────────────────

/// Python-accessible LLM client.
///
/// Wraps `liter_llm::ManagedClient` and exposes async methods that return Python
/// coroutines via `pyo3-async-runtimes`.
///
/// Hooks can be added after construction via [`add_hook`].  The client itself
/// is otherwise immutable; the hook list is protected by an `RwLock`.
#[pyclass(name = "LlmClient")]
pub struct PyLlmClient {
    inner: Arc<ManagedClient>,
    /// Runtime-registered hooks invoked around each request.
    hooks: Arc<RwLock<Vec<Arc<dyn LlmHook>>>>,
    /// Stored for __repr__ display only.
    base_url: Option<String>,
    /// Stored for __repr__ display only.
    max_retries: u32,
}

#[pymethods]
impl PyLlmClient {
    /// Create a new `LlmClient`.
    ///
    /// Args:
    ///     api_key: API key for authentication.
    ///     base_url: Override provider base URL (useful for mock/local servers).
    ///     model_hint: Hint for provider auto-detection (e.g. ``"groq/llama3-70b"``).
    ///         Pass this when no ``base_url`` is set so the client can select the
    ///         correct provider endpoint and auth style at construction time.
    ///     max_retries: Retries on 429 / 5xx.  Defaults to ``3``.
    ///     timeout: Request timeout in seconds.  Defaults to ``60``.
    ///     cache: Optional cache configuration dict with ``max_entries`` and
    ///         ``ttl_seconds`` keys.
    ///     budget: Optional budget configuration dict with ``global_limit``,
    ///         ``model_limits``, and ``enforcement`` keys.
    ///     extra_headers: Optional dict of additional HTTP headers to include
    ///         in every request.
    ///     cooldown: Cooldown period in seconds between requests after errors.
    ///     rate_limit: Optional rate limit configuration dict with ``rpm``
    ///         (requests per minute) and/or ``tpm`` (tokens per minute) keys.
    ///     health_check: Health check interval in seconds.
    ///     cost_tracking: Enable cost tracking middleware.  Defaults to ``False``.
    ///     tracing: Enable tracing middleware.  Defaults to ``False``.
    #[new]
    #[pyo3(signature = (*, api_key, base_url = None, model_hint = None, max_retries = 3, timeout = 60, cache = None, budget = None, extra_headers = None, cooldown = None, rate_limit = None, health_check = None, cost_tracking = false, tracing = false))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        api_key: String,
        base_url: Option<String>,
        model_hint: Option<String>,
        max_retries: u32,
        timeout: u64,
        cache: Option<Bound<'_, PyDict>>,
        budget: Option<Bound<'_, PyDict>>,
        extra_headers: Option<Bound<'_, PyDict>>,
        cooldown: Option<u64>,
        rate_limit: Option<Bound<'_, PyDict>>,
        health_check: Option<u64>,
        cost_tracking: bool,
        tracing: bool,
    ) -> PyResult<Self> {
        let mut builder = ClientConfigBuilder::new(api_key);
        if let Some(ref url) = base_url {
            builder = builder.base_url(url.clone());
        }
        builder = builder.max_retries(max_retries);
        builder = builder.timeout(std::time::Duration::from_secs(timeout));

        // Apply optional cache configuration.
        if let Some(ref cache_dict) = cache {
            let cache_cfg = parse_cache_config(cache_dict)?;
            builder = builder.cache(cache_cfg);
        }

        // Apply optional budget configuration.
        if let Some(ref budget_dict) = budget {
            let budget_cfg = parse_budget_config(budget_dict)?;
            builder = builder.budget(budget_cfg);
        }

        // Apply optional extra headers.
        if let Some(ref headers_dict) = extra_headers {
            for (k, v) in headers_dict.iter() {
                let key: String = k
                    .extract()
                    .map_err(|_| PyValueError::new_err("extra_headers keys must be strings"))?;
                let value: String = v
                    .extract()
                    .map_err(|_| PyValueError::new_err("extra_headers values must be strings"))?;
                builder = builder.header(key, value).map_err(to_py_err)?;
            }
        }

        // Apply optional cooldown.
        if let Some(secs) = cooldown {
            builder = builder.cooldown(std::time::Duration::from_secs(secs));
        }

        // Apply optional rate limit configuration.
        if let Some(ref rl_dict) = rate_limit {
            let rpm: Option<u32> = rl_dict.get_item("rpm")?.map(|v| v.extract::<u32>()).transpose()?;
            let tpm: Option<u64> = rl_dict.get_item("tpm")?.map(|v| v.extract::<u64>()).transpose()?;
            let window_seconds: u64 = rl_dict
                .get_item("window_seconds")?
                .map(|v| v.extract::<u64>())
                .transpose()?
                .unwrap_or(60);
            let rl_config = RateLimitConfig {
                rpm,
                tpm,
                window: std::time::Duration::from_secs(window_seconds),
            };
            builder = builder.rate_limit(rl_config);
        }

        // Apply optional health check interval.
        if let Some(secs) = health_check {
            builder = builder.health_check(std::time::Duration::from_secs(secs));
        }

        // Apply cost tracking and tracing flags.
        if cost_tracking {
            builder = builder.cost_tracking(true);
        }
        if tracing {
            builder = builder.tracing(true);
        }

        let config = builder.build();

        let client = ManagedClient::new(config, model_hint.as_deref()).map_err(to_py_err)?;
        Ok(Self {
            inner: Arc::new(client),
            hooks: Arc::new(RwLock::new(Vec::new())),
            base_url,
            max_retries,
        })
    }

    /// Register a hook object that will be called around each request.
    ///
    /// The hook object may define any combination of these methods (all optional):
    ///
    /// - ``on_request(request: str) -> None`` — called before each request.
    ///   Raise an exception to reject the request.
    /// - ``on_response(request: str, response: str) -> None`` — called after
    ///   a successful response.
    /// - ``on_error(request: str, error: str) -> None`` — called when the
    ///   request fails with an error.
    ///
    /// Methods may be regular functions or ``async def`` coroutines.
    fn add_hook<'py>(&self, py: Python<'py>, hook: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let bridge: Arc<dyn LlmHook> = Arc::new(PyHookBridge::new(hook.unbind()));
        let hooks = Arc::clone(&self.hooks);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            hooks.write().await.push(bridge);
            Ok(())
        })
    }

    /// Register a custom LLM provider in the global provider registry.
    ///
    /// Args:
    ///     config: A dict with ``name`` (str), ``base_url`` (str),
    ///         ``auth_header`` (str — ``"bearer"``, ``"none"``, or
    ///         ``"api-key:<header-name>"``), and ``model_prefixes`` (list[str]).
    ///
    /// The provider will be used for model routing when a model name matches
    /// any of the given prefixes.
    #[staticmethod]
    fn register_provider(config: Bound<'_, PyDict>) -> PyResult<()> {
        let provider_cfg = parse_provider_config(&config)?;
        register_custom_provider(provider_cfg).map_err(to_py_err)
    }

    /// Send a chat completion request (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Chat Completions API.
    /// Returns a coroutine that resolves to a ``ChatCompletionResponse``.
    #[pyo3(signature = (**kwargs))]
    fn chat<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict =
            kwargs.ok_or_else(|| PyValueError::new_err("chat() requires keyword arguments (model, messages, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::ChatCompletionRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.chat(req).await.map_err(to_py_err)?;
            Ok(PyChatCompletionResponse::from(resp))
        })
    }

    /// Start a streaming chat completion.
    ///
    /// Returns an async iterator (``ChatStreamIterator``) that yields
    /// ``ChatCompletionChunk`` objects.  The HTTP request is issued immediately
    /// when this method is called, not on first iteration.
    ///
    /// Use with ``async for chunk in client.chat_stream(**kwargs)``.
    ///
    /// The iterator supports ``async with`` for deterministic resource cleanup:
    /// early exit via ``break`` will signal the background task to stop.
    #[pyo3(signature = (**kwargs))]
    fn chat_stream<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| PyValueError::new_err("chat_stream() requires keyword arguments"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::ChatCompletionRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_bg = Arc::clone(&cancelled);

        // Create the channel before spawning so the receiver is ready before
        // the first __anext__ call.
        let (tx, rx) = mpsc::channel::<liter_llm::Result<liter_llm::ChatCompletionChunk>>(32);
        let rx = Arc::new(Mutex::new(Some(rx)));

        // Spawn the background stream task inside a running Tokio context.
        // future_into_py provides that context; we use a one-shot future here
        // purely to get into the runtime, then immediately return the iterator.
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let handle = tokio::spawn(async move {
                use std::pin::Pin;
                use std::task::Context;

                match client.chat_stream(req).await {
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                    }
                    Ok(mut stream) => loop {
                        if cancelled_bg.load(Ordering::Acquire) {
                            break;
                        }
                        let next = std::future::poll_fn(|cx: &mut Context<'_>| {
                            use futures_core::stream::Stream;
                            Pin::new(&mut stream).poll_next(cx)
                        })
                        .await;
                        match next {
                            Some(item) => {
                                if tx.send(item).await.is_err() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    },
                }
            });

            Ok(PyAsyncChunkIterator {
                rx,
                cancelled,
                handle: Arc::new(Mutex::new(Some(handle))),
            })
        })
    }

    /// Send an embedding request (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Embeddings API.
    /// Returns a coroutine that resolves to an ``EmbeddingResponse``.
    #[pyo3(signature = (**kwargs))]
    fn embed<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict =
            kwargs.ok_or_else(|| PyValueError::new_err("embed() requires keyword arguments (model, input, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::EmbeddingRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.embed(req).await.map_err(to_py_err)?;
            Ok(PyEmbeddingResponse::from(resp))
        })
    }

    /// List available models from the provider (async).
    ///
    /// Returns a coroutine that resolves to a ``ModelsListResponse``.
    fn list_models<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.list_models().await.map_err(to_py_err)?;
            Ok(PyModelsListResponse::from(resp))
        })
    }

    // ─── Additional inference methods ────────────────────────────────────────

    /// Generate images from a text prompt (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Images API.
    /// Returns a coroutine that resolves to an ``ImagesResponse``.
    #[pyo3(signature = (**kwargs))]
    fn image_generate<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict =
            kwargs.ok_or_else(|| PyValueError::new_err("image_generate() requires keyword arguments (prompt, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateImageRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.image_generate(req).await.map_err(to_py_err)?;
            Ok(PyImagesResponse::from(resp))
        })
    }

    /// Generate speech audio from text (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Audio Speech API.
    /// Returns a coroutine that resolves to ``bytes`` containing the audio data.
    #[pyo3(signature = (**kwargs))]
    fn speech<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs
            .ok_or_else(|| PyValueError::new_err("speech() requires keyword arguments (model, input, voice, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateSpeechRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.speech(req).await.map_err(to_py_err)?;
            Ok(resp.to_vec())
        })
    }

    /// Transcribe audio into text (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Audio Transcription API.
    /// Returns a coroutine that resolves to a ``TranscriptionResponse``.
    #[pyo3(signature = (**kwargs))]
    fn transcribe<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs
            .ok_or_else(|| PyValueError::new_err("transcribe() requires keyword arguments (model, file, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateTranscriptionRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.transcribe(req).await.map_err(to_py_err)?;
            Ok(PyTranscriptionResponse::from(resp))
        })
    }

    /// Classify content for policy violations (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Moderations API.
    /// Returns a coroutine that resolves to a ``ModerationResponse``.
    #[pyo3(signature = (**kwargs))]
    fn moderate<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| PyValueError::new_err("moderate() requires keyword arguments (input, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::ModerationRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.moderate(req).await.map_err(to_py_err)?;
            Ok(PyModerationResponse::from(resp))
        })
    }

    /// Rerank documents by relevance to a query (async).
    ///
    /// Accepts the same keyword arguments as the Cohere/Jina rerank API.
    /// Returns a coroutine that resolves to a ``RerankResponse``.
    #[pyo3(signature = (**kwargs))]
    fn rerank<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| {
            PyValueError::new_err("rerank() requires keyword arguments (model, query, documents, ...)")
        })?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::RerankRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.rerank(req).await.map_err(to_py_err)?;
            Ok(PyRerankResponse::from(resp))
        })
    }

    /// Perform a web/document search (async).
    ///
    /// Accepts the same keyword arguments as the search API.
    /// Returns a coroutine that resolves to a ``dict`` with search results.
    #[pyo3(signature = (**kwargs))]
    fn search<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict =
            kwargs.ok_or_else(|| PyValueError::new_err("search() requires keyword arguments (model, query, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::SearchRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.search(req).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Extract text from a document via OCR (async).
    ///
    /// Accepts the same keyword arguments as the OCR API.
    /// Returns a coroutine that resolves to a ``dict`` with OCR results.
    #[pyo3(signature = (**kwargs))]
    fn ocr<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict =
            kwargs.ok_or_else(|| PyValueError::new_err("ocr() requires keyword arguments (model, document, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::OcrRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.ocr(req).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    // ─── File management methods ─────────────────────────────────────────────

    /// Upload a file (async).
    ///
    /// Returns a coroutine that resolves to a ``dict`` with file object fields.
    #[pyo3(signature = (**kwargs))]
    fn create_file<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs
            .ok_or_else(|| PyValueError::new_err("create_file() requires keyword arguments (file, purpose, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateFileRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.create_file(req).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Retrieve metadata about an uploaded file (async).
    ///
    /// Args:
    ///     file_id: The ID of the file to retrieve.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with file object fields.
    fn retrieve_file<'py>(&self, py: Python<'py>, file_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.retrieve_file(&file_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Delete an uploaded file (async).
    ///
    /// Args:
    ///     file_id: The ID of the file to delete.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with deletion status.
    fn delete_file<'py>(&self, py: Python<'py>, file_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.delete_file(&file_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// List uploaded files (async).
    ///
    /// Optional keyword arguments: ``purpose``, ``limit``, ``after``.
    /// Returns a coroutine that resolves to a ``dict`` with a ``data`` list.
    #[pyo3(signature = (**kwargs))]
    fn list_files<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let query: Option<liter_llm::FileListQuery> = match kwargs {
            Some(ref dict) if !dict.is_empty() => {
                let value = kwargs_to_json(dict)?;
                Some(serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?)
            }
            _ => None,
        };

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.list_files(query).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Download the content of an uploaded file (async).
    ///
    /// Args:
    ///     file_id: The ID of the file whose content to download.
    ///
    /// Returns a coroutine that resolves to ``bytes``.
    fn file_content<'py>(&self, py: Python<'py>, file_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.file_content(&file_id).await.map_err(to_py_err)?;
            Ok(resp.to_vec())
        })
    }

    // ─── Batch management methods ────────────────────────────────────────────

    /// Create a new batch (async).
    ///
    /// Returns a coroutine that resolves to a ``dict`` with batch object fields.
    #[pyo3(signature = (**kwargs))]
    fn create_batch<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| {
            PyValueError::new_err(
                "create_batch() requires keyword arguments (input_file_id, endpoint, completion_window, ...)",
            )
        })?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateBatchRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.create_batch(req).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Retrieve a batch by ID (async).
    ///
    /// Args:
    ///     batch_id: The ID of the batch to retrieve.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with batch object fields.
    fn retrieve_batch<'py>(&self, py: Python<'py>, batch_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.retrieve_batch(&batch_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// List batches (async).
    ///
    /// Optional keyword arguments: ``limit``, ``after``.
    /// Returns a coroutine that resolves to a ``dict`` with a ``data`` list.
    #[pyo3(signature = (**kwargs))]
    fn list_batches<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let query: Option<liter_llm::BatchListQuery> = match kwargs {
            Some(ref dict) if !dict.is_empty() => {
                let value = kwargs_to_json(dict)?;
                Some(serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?)
            }
            _ => None,
        };

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.list_batches(query).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Cancel a batch (async).
    ///
    /// Args:
    ///     batch_id: The ID of the batch to cancel.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with batch object fields.
    fn cancel_batch<'py>(&self, py: Python<'py>, batch_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.cancel_batch(&batch_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    // ─── Response management methods ─────────────────────────────────────────

    /// Create a new response (async).
    ///
    /// Returns a coroutine that resolves to a ``dict`` with response object fields.
    #[pyo3(signature = (**kwargs))]
    fn create_response<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs
            .ok_or_else(|| PyValueError::new_err("create_response() requires keyword arguments (model, input, ...)"))?;
        let value = kwargs_to_json(&dict)?;
        let req: liter_llm::CreateResponseRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.create_response(req).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Retrieve a response by ID (async).
    ///
    /// Args:
    ///     response_id: The ID of the response to retrieve.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with response object fields.
    fn retrieve_response<'py>(&self, py: Python<'py>, response_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.retrieve_response(&response_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Cancel a response (async).
    ///
    /// Args:
    ///     response_id: The ID of the response to cancel.
    ///
    /// Returns a coroutine that resolves to a ``dict`` with response object fields.
    fn cancel_response<'py>(&self, py: Python<'py>, response_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.cancel_response(&response_id).await.map_err(to_py_err)?;
            to_json_value(&resp)
        })
    }

    /// Return the total budget spend so far (in USD).
    ///
    /// Returns ``0.0`` if no budget is configured.
    #[getter]
    fn budget_used(&self) -> f64 {
        self.inner.budget_state().map(|s| s.global_spend()).unwrap_or(0.0)
    }

    /// Unregister a previously registered custom provider by name.
    ///
    /// Args:
    ///     name: The provider name to unregister.
    ///
    /// Returns:
    ///     ``True`` if the provider was found and removed, ``False`` otherwise.
    #[staticmethod]
    fn unregister_provider(name: String) -> PyResult<bool> {
        unregister_custom_provider(&name).map_err(to_py_err)
    }

    fn __repr__(&self) -> String {
        match &self.base_url {
            Some(url) => format!("LlmClient(base_url={url:?}, max_retries={})", self.max_retries),
            None => format!("LlmClient(max_retries={})", self.max_retries),
        }
    }
}

// ─── Async iterator for streaming ────────────────────────────────────────────

type ChunkRx = mpsc::Receiver<liter_llm::Result<liter_llm::ChatCompletionChunk>>;
type StreamHandle = JoinHandle<()>;

/// Async iterator that yields `ChatCompletionChunk` objects.
///
/// Obtain via `LlmClient.chat_stream(**kwargs)`.  The underlying HTTP stream is
/// started immediately when `chat_stream` is called; this object is the consumer
/// side of the channel.
///
/// Supports ``async with`` for deterministic cleanup: the context manager
/// signals the background producer task to stop on exit.
#[pyclass(name = "ChatStreamIterator")]
pub struct PyAsyncChunkIterator {
    /// The channel receiver.  Wrapped in `Option` so we can take it out on
    /// first call without holding a lock across await points.
    pub(crate) rx: Arc<Mutex<Option<ChunkRx>>>,
    /// Set to `true` to ask the background task to stop.
    cancelled: Arc<AtomicBool>,
    /// JoinHandle for the background producer task.  Awaited in `__aexit__`
    /// to surface any panics that occurred inside the background task.
    handle: Arc<Mutex<Option<StreamHandle>>>,
}

#[pymethods]
impl PyAsyncChunkIterator {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = Arc::clone(&self.rx);
        let cancelled = Arc::clone(&self.cancelled);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Check cancellation BEFORE acquiring the lock so that __aexit__
            // (which sets cancelled=true then locks rx to drop it) can never
            // deadlock with a concurrent __anext__ that is holding the lock
            // across an await point.
            if cancelled.load(Ordering::Acquire) {
                return Err(PyStopAsyncIteration::new_err(()));
            }

            let mut guard = rx.lock().await;
            let receiver = guard.as_mut().ok_or_else(|| PyStopAsyncIteration::new_err(()))?;

            match receiver.recv().await {
                Some(Ok(chunk)) => Ok(PyChatCompletionChunk::from(chunk)),
                Some(Err(e)) => Err(to_py_err(e)),
                None => Err(PyStopAsyncIteration::new_err(())),
            }
        })
    }

    /// Enter the async context manager.  Returns `self`.
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // Convert to an unbound `Py<T>` (which is `Send`) before entering the
        // async future.  The future re-attaches the GIL on return.
        let pyobj: Py<PyAsyncChunkIterator> = slf.into_pyobject(py)?.unbind();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { Ok(pyobj) })
    }

    /// Exit the async context manager.  Signals the background task to stop
    /// and awaits the JoinHandle to surface any panics from the background task.
    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: Option<Bound<'py, PyAny>>,
        _exc_val: Option<Bound<'py, PyAny>>,
        _exc_tb: Option<Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // Signal the producer to stop and drain/close the receiver so the
        // background task's sends fail fast.
        self.cancelled.store(true, Ordering::Release);
        let rx = Arc::clone(&self.rx);
        let handle = Arc::clone(&self.handle);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Drop the receiver to unblock any pending send in the background task.
            *rx.lock().await = None;
            // Await the background task to propagate any panics it may have raised.
            if let Some(jh) = handle.lock().await.take() {
                // A panic in the background task shows up as Err(JoinError).
                // Surface it as a Python RuntimeError rather than silently dropping it.
                jh.await.map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!("background stream task panicked: {e}"))
                })?;
            }
            Ok(false) // do not suppress exceptions
        })
    }

    /// Signal the background stream task to stop.
    ///
    /// Called automatically by ``__aexit__``.  Can also be called manually
    /// when the iterator is discarded early.
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

impl Drop for PyAsyncChunkIterator {
    fn drop(&mut self) {
        // Best-effort cancellation signal when the Python object is GC'd
        // without going through __aexit__.  The background task checks this
        // flag on every iteration and will stop on its next loop.
        self.cancelled.store(true, Ordering::Release);
    }
}
