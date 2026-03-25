use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use liter_lm::{ClientConfigBuilder, DefaultClient, LlmClient};
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;

use crate::error::to_py_err;
use crate::types::{PyChatCompletionChunk, PyChatCompletionResponse, PyEmbeddingResponse, PyModelsListResponse};

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
/// Wraps `liter_lm::DefaultClient` and exposes async methods that return Python
/// coroutines via `pyo3-async-runtimes`.
///
/// `LlmClient` is immutable after construction and safe to share across threads.
#[pyclass(frozen, name = "LlmClient")]
pub struct PyLlmClient {
    inner: Arc<DefaultClient>,
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
    #[new]
    #[pyo3(signature = (*, api_key, base_url = None, model_hint = None, max_retries = 3, timeout = 60))]
    fn new(
        api_key: String,
        base_url: Option<String>,
        model_hint: Option<String>,
        max_retries: u32,
        timeout: u64,
    ) -> PyResult<Self> {
        let mut builder = ClientConfigBuilder::new(api_key);
        if let Some(ref url) = base_url {
            builder = builder.base_url(url.clone());
        }
        builder = builder.max_retries(max_retries);
        builder = builder.timeout(std::time::Duration::from_secs(timeout));
        let config = builder.build();

        let client = DefaultClient::new(config, model_hint.as_deref()).map_err(to_py_err)?;
        Ok(Self {
            inner: Arc::new(client),
            base_url,
            max_retries,
        })
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
        let req: liter_lm::ChatCompletionRequest =
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
        let req: liter_lm::ChatCompletionRequest =
            serde_json::from_value(value).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_bg = Arc::clone(&cancelled);

        // Create the channel before spawning so the receiver is ready before
        // the first __anext__ call.
        let (tx, rx) = mpsc::channel::<liter_lm::Result<liter_lm::ChatCompletionChunk>>(32);
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
        let req: liter_lm::EmbeddingRequest =
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

    fn __repr__(&self) -> String {
        match &self.base_url {
            Some(url) => format!("LlmClient(base_url={url:?}, max_retries={})", self.max_retries),
            None => format!("LlmClient(max_retries={})", self.max_retries),
        }
    }
}

// ─── Async iterator for streaming ────────────────────────────────────────────

type ChunkRx = mpsc::Receiver<liter_lm::Result<liter_lm::ChatCompletionChunk>>;
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
