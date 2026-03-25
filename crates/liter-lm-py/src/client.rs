use std::sync::Arc;

use liter_lm::{ClientConfigBuilder, DefaultClient, LlmClient};
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::sync::{Mutex, mpsc};

use crate::error::to_py_err;
use crate::types::{PyChatCompletionChunk, PyChatCompletionResponse, PyEmbeddingResponse, PyModelsListResponse};

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Convert a Python dict (kwargs) into a `serde_json::Value`.
///
/// Round-trips through the Python `json` module to handle all JSON-serialisable
/// Python types (nested dicts, lists, None, etc.) without manual translation.
fn kwargs_to_json(py: Python<'_>, kwargs: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let json_mod = py.import("json")?;
    let json_str: String = json_mod.call_method1("dumps", (kwargs.as_unbound(),))?.extract()?;
    serde_json::from_str(&json_str).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

// ─── LlmClient Python class ───────────────────────────────────────────────────

/// Python-accessible LLM client.
///
/// Wraps `liter_lm::DefaultClient` and exposes async methods that return Python
/// coroutines via `pyo3-async-runtimes`.
#[pyclass(name = "LlmClient")]
pub struct PyLlmClient {
    inner: Arc<DefaultClient>,
}

#[pymethods]
impl PyLlmClient {
    /// Create a new `LlmClient`.
    ///
    /// Args:
    ///     api_key: API key for authentication.
    ///     base_url: Override provider base URL (useful for mock/local servers).
    ///     max_retries: Retries on 429 / 5xx.  Defaults to ``3``.
    ///     timeout: Request timeout in seconds.  Defaults to ``60``.
    #[new]
    #[pyo3(signature = (*, api_key, base_url = None, max_retries = 3, timeout = 60))]
    fn new(api_key: String, base_url: Option<String>, max_retries: u32, timeout: u64) -> PyResult<Self> {
        let mut builder = ClientConfigBuilder::new(api_key);
        if let Some(url) = base_url {
            builder = builder.base_url(url);
        }
        builder = builder.max_retries(max_retries);
        builder = builder.timeout(std::time::Duration::from_secs(timeout));
        let config = builder.build();

        let client = DefaultClient::new(config, None).map_err(to_py_err)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Send a chat completion request (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Chat Completions API.
    /// Returns a coroutine that resolves to a ``ChatCompletionResponse``.
    #[pyo3(signature = (**kwargs))]
    fn chat<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("chat() requires keyword arguments (model, messages, ...)")
        })?;
        let value = kwargs_to_json(py, &dict)?;
        let req: liter_lm::ChatCompletionRequest =
            serde_json::from_value(value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp = client.chat(req).await.map_err(to_py_err)?;
            Ok(PyChatCompletionResponse::from(resp))
        })
    }

    /// Start a streaming chat completion.
    ///
    /// Returns an async iterator that yields ``ChatCompletionChunk`` objects.
    /// Use with ``async for chunk in client.chat_stream(**kwargs)``.
    ///
    /// The stream setup (HTTP request) is initiated on the first iteration.
    #[pyo3(signature = (**kwargs))]
    fn chat_stream(&self, py: Python<'_>, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<PyAsyncChunkIterator> {
        let dict = kwargs
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("chat_stream() requires keyword arguments"))?;
        let value = kwargs_to_json(py, &dict)?;
        let req: liter_lm::ChatCompletionRequest =
            serde_json::from_value(value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(PyAsyncChunkIterator {
            client: Arc::clone(&self.inner),
            req: std::sync::Mutex::new(Some(req)),
            rx: Arc::new(Mutex::new(None)),
        })
    }

    /// Send an embedding request (async).
    ///
    /// Accepts the same keyword arguments as the OpenAI Embeddings API.
    /// Returns a coroutine that resolves to an ``EmbeddingResponse``.
    #[pyo3(signature = (**kwargs))]
    fn embed<'py>(&self, py: Python<'py>, kwargs: Option<Bound<'py, PyDict>>) -> PyResult<Bound<'py, PyAny>> {
        let dict = kwargs.ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("embed() requires keyword arguments (model, input, ...)")
        })?;
        let value = kwargs_to_json(py, &dict)?;
        let req: liter_lm::EmbeddingRequest =
            serde_json::from_value(value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

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

    fn __repr__(&self) -> &'static str {
        "LlmClient(...)"
    }
}

// ─── Async iterator for streaming ────────────────────────────────────────────

type ChunkRx = mpsc::Receiver<liter_lm::Result<liter_lm::ChatCompletionChunk>>;

/// Async iterator that yields `ChatCompletionChunk` objects.
///
/// The underlying stream is started lazily on the first `__anext__` call so
/// that the iterator can be constructed synchronously without requiring an
/// active Tokio runtime.
#[pyclass(name = "ChatStreamIterator")]
pub struct PyAsyncChunkIterator {
    /// The client used to start the stream.
    client: Arc<DefaultClient>,
    /// The request; taken once when the stream is first started.
    req: std::sync::Mutex<Option<liter_lm::ChatCompletionRequest>>,
    /// The channel receiver once the stream has been started.
    rx: Arc<Mutex<Option<ChunkRx>>>,
}

#[pymethods]
impl PyAsyncChunkIterator {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = Arc::clone(&self.rx);
        let client = Arc::clone(&self.client);

        // Take the request out of the Option.  This is Some only on the first
        // call; subsequent calls get None and the already-running background
        // task keeps feeding the channel.
        let maybe_req = self.req.lock().expect("req mutex poisoned").take();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Start the background stream task on the first call.
            if let Some(req) = maybe_req {
                let (tx, new_rx) = mpsc::channel::<liter_lm::Result<liter_lm::ChatCompletionChunk>>(32);

                // Store the receiver before spawning so that subsequent
                // `__anext__` calls can find it.
                *rx.lock().await = Some(new_rx);

                // Spawn the background task now that we are inside a Tokio
                // runtime (provided by `future_into_py`).
                tokio::spawn(async move {
                    use std::pin::Pin;
                    use std::task::Context;

                    match client.chat_stream(req).await {
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                        }
                        Ok(mut stream) => loop {
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
            }

            // Receive the next chunk from the channel.
            let mut guard = rx.lock().await;
            let receiver = guard.as_mut().ok_or_else(|| {
                // This shouldn't happen in normal usage.
                PyStopAsyncIteration::new_err(())
            })?;

            match receiver.recv().await {
                Some(Ok(chunk)) => Ok(PyChatCompletionChunk::from(chunk)),
                Some(Err(e)) => Err(to_py_err(e)),
                None => Err(PyStopAsyncIteration::new_err(())),
            }
        })
    }
}
