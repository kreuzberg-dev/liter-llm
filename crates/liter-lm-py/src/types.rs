use pyo3::prelude::*;

// ─── Usage ────────────────────────────────────────────────────────────────────

/// Token usage information for a request.
#[pyclass(frozen, name = "Usage")]
pub struct PyUsage {
    inner: liter_lm::types::Usage,
}

#[pymethods]
impl PyUsage {
    /// Tokens consumed by the prompt.
    #[getter]
    fn prompt_tokens(&self) -> u64 {
        self.inner.prompt_tokens
    }

    /// Tokens consumed by the completion.
    #[getter]
    fn completion_tokens(&self) -> u64 {
        self.inner.completion_tokens
    }

    /// Total tokens (prompt + completion).
    #[getter]
    fn total_tokens(&self) -> u64 {
        self.inner.total_tokens
    }

    fn __repr__(&self) -> String {
        format!(
            "Usage(prompt_tokens={}, completion_tokens={}, total_tokens={})",
            self.inner.prompt_tokens, self.inner.completion_tokens, self.inner.total_tokens
        )
    }
}

impl From<liter_lm::types::Usage> for PyUsage {
    fn from(inner: liter_lm::types::Usage) -> Self {
        Self { inner }
    }
}

// ─── Tool call ────────────────────────────────────────────────────────────────

/// Function information within a tool call.
#[pyclass(frozen, name = "FunctionCall")]
pub struct PyFunctionCall {
    inner: liter_lm::types::FunctionCall,
}

#[pymethods]
impl PyFunctionCall {
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    #[getter]
    fn arguments(&self) -> &str {
        &self.inner.arguments
    }

    fn __repr__(&self) -> String {
        format!(
            "FunctionCall(name={:?}, arguments={:?})",
            self.inner.name, self.inner.arguments
        )
    }
}

/// A tool call made by the assistant.
#[pyclass(frozen, name = "ToolCall")]
pub struct PyToolCall {
    inner: liter_lm::types::ToolCall,
}

#[pymethods]
impl PyToolCall {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    #[getter]
    fn function(&self) -> PyFunctionCall {
        PyFunctionCall {
            inner: self.inner.function.clone(),
        }
    }

    fn __repr__(&self) -> String {
        format!("ToolCall(id={:?})", self.inner.id)
    }
}

// ─── AssistantMessage ─────────────────────────────────────────────────────────

/// The message field of a chat completion choice.
#[pyclass(frozen, name = "AssistantMessage")]
pub struct PyAssistantMessage {
    inner: liter_lm::types::AssistantMessage,
}

#[pymethods]
impl PyAssistantMessage {
    /// Text content of the message, or `None` if the response has tool calls.
    #[getter]
    fn content(&self) -> Option<&str> {
        self.inner.content.as_deref()
    }

    /// Tool calls made by the assistant, or `None`.
    #[getter]
    fn tool_calls(&self) -> Option<Vec<PyToolCall>> {
        self.inner
            .tool_calls
            .as_ref()
            .map(|calls| calls.iter().map(|tc| PyToolCall { inner: tc.clone() }).collect())
    }

    fn __repr__(&self) -> String {
        format!("AssistantMessage(content={:?})", self.inner.content)
    }
}

impl From<liter_lm::types::AssistantMessage> for PyAssistantMessage {
    fn from(inner: liter_lm::types::AssistantMessage) -> Self {
        Self { inner }
    }
}

// ─── Choice ───────────────────────────────────────────────────────────────────

/// A single choice in a chat completion response.
#[pyclass(frozen, name = "Choice")]
pub struct PyChoice {
    inner: liter_lm::types::Choice,
}

#[pymethods]
impl PyChoice {
    /// The message returned by the model.
    #[getter]
    fn message(&self) -> PyAssistantMessage {
        PyAssistantMessage {
            inner: self.inner.message.clone(),
        }
    }

    /// Why the model stopped generating tokens.
    #[getter]
    fn finish_reason(&self) -> Option<String> {
        self.inner.finish_reason.as_ref().map(|r| {
            // Serialize back to the snake_case string the API uses.
            match r {
                liter_lm::types::FinishReason::Stop => "stop".to_owned(),
                liter_lm::types::FinishReason::Length => "length".to_owned(),
                liter_lm::types::FinishReason::ToolCalls => "tool_calls".to_owned(),
                liter_lm::types::FinishReason::ContentFilter => "content_filter".to_owned(),
                liter_lm::types::FinishReason::FunctionCall => "function_call".to_owned(),
            }
        })
    }

    /// Zero-based index of this choice.
    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }

    fn __repr__(&self) -> String {
        format!(
            "Choice(index={}, finish_reason={:?})",
            self.inner.index,
            self.finish_reason()
        )
    }
}

impl From<liter_lm::types::Choice> for PyChoice {
    fn from(inner: liter_lm::types::Choice) -> Self {
        Self { inner }
    }
}

// ─── ChatCompletionResponse ────────────────────────────────────────────────────

/// Response from a chat completion request.
#[pyclass(frozen, name = "ChatCompletionResponse")]
pub struct PyChatCompletionResponse {
    inner: liter_lm::types::ChatCompletionResponse,
}

#[pymethods]
impl PyChatCompletionResponse {
    /// Unique identifier for this completion.
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    /// The model that generated this response.
    #[getter]
    fn model(&self) -> &str {
        &self.inner.model
    }

    /// List of generated choices.
    #[getter]
    fn choices(&self) -> Vec<PyChoice> {
        self.inner
            .choices
            .iter()
            .map(|c| PyChoice { inner: c.clone() })
            .collect()
    }

    /// Token usage information.
    #[getter]
    fn usage(&self) -> Option<PyUsage> {
        self.inner.usage.clone().map(PyUsage::from)
    }

    fn __repr__(&self) -> String {
        format!(
            "ChatCompletionResponse(id={:?}, model={:?})",
            self.inner.id, self.inner.model
        )
    }
}

impl From<liter_lm::types::ChatCompletionResponse> for PyChatCompletionResponse {
    fn from(inner: liter_lm::types::ChatCompletionResponse) -> Self {
        Self { inner }
    }
}

// ─── Streaming chunk types ────────────────────────────────────────────────────

/// The delta object in a streaming chunk choice.
#[pyclass(frozen, name = "StreamDelta")]
pub struct PyStreamDelta {
    inner: liter_lm::types::StreamDelta,
}

#[pymethods]
impl PyStreamDelta {
    /// Partial content token, or `None` if not present in this chunk.
    #[getter]
    fn content(&self) -> Option<&str> {
        self.inner.content.as_deref()
    }

    /// Role, present only in the first chunk.
    #[getter]
    fn role(&self) -> Option<&str> {
        self.inner.role.as_deref()
    }

    fn __repr__(&self) -> String {
        format!("StreamDelta(content={:?})", self.inner.content)
    }
}

/// A single choice within a streaming chunk.
#[pyclass(frozen, name = "StreamChoice")]
pub struct PyStreamChoice {
    inner: liter_lm::types::StreamChoice,
}

#[pymethods]
impl PyStreamChoice {
    #[getter]
    fn delta(&self) -> PyStreamDelta {
        PyStreamDelta {
            inner: self.inner.delta.clone(),
        }
    }

    #[getter]
    fn finish_reason(&self) -> Option<String> {
        self.inner.finish_reason.as_ref().map(|r| match r {
            liter_lm::types::FinishReason::Stop => "stop".to_owned(),
            liter_lm::types::FinishReason::Length => "length".to_owned(),
            liter_lm::types::FinishReason::ToolCalls => "tool_calls".to_owned(),
            liter_lm::types::FinishReason::ContentFilter => "content_filter".to_owned(),
            liter_lm::types::FinishReason::FunctionCall => "function_call".to_owned(),
        })
    }

    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }
}

/// One SSE event from a streaming chat completion.
#[pyclass(frozen, name = "ChatCompletionChunk")]
pub struct PyChatCompletionChunk {
    inner: liter_lm::types::ChatCompletionChunk,
}

#[pymethods]
impl PyChatCompletionChunk {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    #[getter]
    fn model(&self) -> &str {
        &self.inner.model
    }

    #[getter]
    fn choices(&self) -> Vec<PyStreamChoice> {
        self.inner
            .choices
            .iter()
            .map(|c| PyStreamChoice { inner: c.clone() })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "ChatCompletionChunk(id={:?}, model={:?})",
            self.inner.id, self.inner.model
        )
    }
}

impl From<liter_lm::types::ChatCompletionChunk> for PyChatCompletionChunk {
    fn from(inner: liter_lm::types::ChatCompletionChunk) -> Self {
        Self { inner }
    }
}

// ─── Embedding types ──────────────────────────────────────────────────────────

/// A single embedding vector.
#[pyclass(frozen, name = "EmbeddingObject")]
pub struct PyEmbeddingObject {
    inner: liter_lm::types::EmbeddingObject,
}

#[pymethods]
impl PyEmbeddingObject {
    /// The embedding vector.
    #[getter]
    fn embedding(&self) -> Vec<f64> {
        self.inner.embedding.clone()
    }

    /// Index of this embedding in the input list.
    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }

    fn __repr__(&self) -> String {
        format!(
            "EmbeddingObject(index={}, dims={})",
            self.inner.index,
            self.inner.embedding.len()
        )
    }
}

/// Response from an embedding request.
#[pyclass(frozen, name = "EmbeddingResponse")]
pub struct PyEmbeddingResponse {
    inner: liter_lm::types::EmbeddingResponse,
}

#[pymethods]
impl PyEmbeddingResponse {
    /// The model used for embedding.
    #[getter]
    fn model(&self) -> &str {
        &self.inner.model
    }

    /// List of embedding objects.
    #[getter]
    fn data(&self) -> Vec<PyEmbeddingObject> {
        self.inner
            .data
            .iter()
            .map(|o| PyEmbeddingObject { inner: o.clone() })
            .collect()
    }

    /// Token usage for this request.
    #[getter]
    fn usage(&self) -> PyUsage {
        PyUsage {
            inner: self.inner.usage.clone(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "EmbeddingResponse(model={:?}, count={})",
            self.inner.model,
            self.inner.data.len()
        )
    }
}

impl From<liter_lm::types::EmbeddingResponse> for PyEmbeddingResponse {
    fn from(inner: liter_lm::types::EmbeddingResponse) -> Self {
        Self { inner }
    }
}

// ─── Models list types ────────────────────────────────────────────────────────

/// Information about a single model.
#[pyclass(frozen, name = "ModelObject")]
pub struct PyModelObject {
    inner: liter_lm::types::ModelObject,
}

#[pymethods]
impl PyModelObject {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    #[getter]
    fn owned_by(&self) -> &str {
        &self.inner.owned_by
    }

    #[getter]
    fn created(&self) -> u64 {
        self.inner.created
    }

    fn __repr__(&self) -> String {
        format!("ModelObject(id={:?})", self.inner.id)
    }
}

/// Response from a list-models request.
#[pyclass(frozen, name = "ModelsListResponse")]
pub struct PyModelsListResponse {
    inner: liter_lm::types::ModelsListResponse,
}

#[pymethods]
impl PyModelsListResponse {
    /// List of available models.
    #[getter]
    fn data(&self) -> Vec<PyModelObject> {
        self.inner
            .data
            .iter()
            .map(|m| PyModelObject { inner: m.clone() })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!("ModelsListResponse(count={})", self.inner.data.len())
    }
}

impl From<liter_lm::types::ModelsListResponse> for PyModelsListResponse {
    fn from(inner: liter_lm::types::ModelsListResponse) -> Self {
        Self { inner }
    }
}

/// Register all response types on the module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUsage>()?;
    m.add_class::<PyFunctionCall>()?;
    m.add_class::<PyToolCall>()?;
    m.add_class::<PyAssistantMessage>()?;
    m.add_class::<PyChoice>()?;
    m.add_class::<PyChatCompletionResponse>()?;
    m.add_class::<PyStreamDelta>()?;
    m.add_class::<PyStreamChoice>()?;
    m.add_class::<PyChatCompletionChunk>()?;
    m.add_class::<PyEmbeddingObject>()?;
    m.add_class::<PyEmbeddingResponse>()?;
    m.add_class::<PyModelObject>()?;
    m.add_class::<PyModelsListResponse>()?;
    Ok(())
}
