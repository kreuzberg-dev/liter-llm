use pyo3::prelude::*;

// ─── Shared helper ────────────────────────────────────────────────────────────

/// Convert a [`liter_llm::types::FinishReason`] to its canonical snake_case
/// string representation, reusing the serde rename metadata so this function
/// never diverges from the wire format.
fn finish_reason_str(r: &liter_llm::types::FinishReason) -> String {
    // serde_json serialises the enum using its `#[serde(rename_all = "snake_case")]`
    // annotations, so we get the canonical wire string without a manual match.
    serde_json::to_value(r)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_else(|| "other".to_owned())
}

// ─── Usage ────────────────────────────────────────────────────────────────────

/// Token usage information for a request.
#[pyclass(frozen, skip_from_py_object, name = "Usage")]
#[derive(Clone)]
pub struct PyUsage {
    inner: liter_llm::types::Usage,
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

    fn __eq__(&self, other: &PyUsage) -> bool {
        self.inner.prompt_tokens == other.inner.prompt_tokens
            && self.inner.completion_tokens == other.inner.completion_tokens
            && self.inner.total_tokens == other.inner.total_tokens
    }
}

impl From<liter_llm::types::Usage> for PyUsage {
    fn from(inner: liter_llm::types::Usage) -> Self {
        Self { inner }
    }
}

// ─── Tool call ────────────────────────────────────────────────────────────────

/// Function information within a tool call.
#[pyclass(frozen, name = "FunctionCall")]
pub struct PyFunctionCall {
    inner: liter_llm::types::FunctionCall,
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

    fn __eq__(&self, other: &PyFunctionCall) -> bool {
        self.inner.name == other.inner.name && self.inner.arguments == other.inner.arguments
    }
}

/// A tool call made by the assistant.
#[pyclass(frozen, name = "ToolCall")]
pub struct PyToolCall {
    inner: liter_llm::types::ToolCall,
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
        format!(
            "ToolCall(id={:?}, function={})",
            self.inner.id,
            PyFunctionCall {
                inner: self.inner.function.clone()
            }
            .__repr__()
        )
    }

    fn __eq__(&self, other: &PyToolCall) -> bool {
        self.inner.id == other.inner.id
            && self.inner.function.name == other.inner.function.name
            && self.inner.function.arguments == other.inner.function.arguments
    }
}

// ─── AssistantMessage ─────────────────────────────────────────────────────────

/// The message field of a chat completion choice.
#[pyclass(frozen, name = "AssistantMessage")]
pub struct PyAssistantMessage {
    inner: liter_llm::types::AssistantMessage,
}

#[pymethods]
impl PyAssistantMessage {
    /// Text content of the message, or `None` if the response has tool calls.
    #[getter]
    fn content(&self) -> Option<&str> {
        self.inner.content.as_deref()
    }

    /// The assistant's optional display name.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    /// Refusal message set by the model, or `None`.
    #[getter]
    fn refusal(&self) -> Option<&str> {
        self.inner.refusal.as_deref()
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
        format!(
            "AssistantMessage(content={:?}, refusal={:?}, name={:?})",
            self.inner.content, self.inner.refusal, self.inner.name
        )
    }

    fn __eq__(&self, other: &PyAssistantMessage) -> bool {
        self.inner.content == other.inner.content
            && self.inner.name == other.inner.name
            && self.inner.refusal == other.inner.refusal
            && self.inner.tool_calls == other.inner.tool_calls
    }
}

impl From<liter_llm::types::AssistantMessage> for PyAssistantMessage {
    fn from(inner: liter_llm::types::AssistantMessage) -> Self {
        Self { inner }
    }
}

// ─── Choice ───────────────────────────────────────────────────────────────────

/// A single choice in a chat completion response.
#[pyclass(frozen, name = "Choice")]
pub struct PyChoice {
    inner: liter_llm::types::Choice,
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
        self.inner.finish_reason.as_ref().map(finish_reason_str)
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

    fn __eq__(&self, other: &PyChoice) -> bool {
        self.inner.index == other.inner.index
            && self.inner.finish_reason == other.inner.finish_reason
            && self.inner.message == other.inner.message
    }
}

impl From<liter_llm::types::Choice> for PyChoice {
    fn from(inner: liter_llm::types::Choice) -> Self {
        Self { inner }
    }
}

// ─── ChatCompletionResponse ────────────────────────────────────────────────────

/// Response from a chat completion request.
#[pyclass(frozen, name = "ChatCompletionResponse")]
pub struct PyChatCompletionResponse {
    inner: liter_llm::types::ChatCompletionResponse,
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

    /// Unix timestamp of when the response was created.
    #[getter]
    fn created(&self) -> u64 {
        self.inner.created
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

    /// System fingerprint for reproducibility, if provided by the backend.
    #[getter]
    fn system_fingerprint(&self) -> Option<&str> {
        self.inner.system_fingerprint.as_deref()
    }

    /// Service tier used for this request, if returned by the backend.
    #[getter]
    fn service_tier(&self) -> Option<&str> {
        self.inner.service_tier.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "ChatCompletionResponse(id={:?}, model={:?}, created={}, choices={})",
            self.inner.id,
            self.inner.model,
            self.inner.created,
            self.inner.choices.len()
        )
    }

    fn __eq__(&self, other: &PyChatCompletionResponse) -> bool {
        self.inner.id == other.inner.id
            && self.inner.model == other.inner.model
            && self.inner.created == other.inner.created
            && self.inner.choices == other.inner.choices
            && self.inner.usage == other.inner.usage
    }
}

impl From<liter_llm::types::ChatCompletionResponse> for PyChatCompletionResponse {
    fn from(inner: liter_llm::types::ChatCompletionResponse) -> Self {
        Self { inner }
    }
}

// ─── Streaming chunk types ────────────────────────────────────────────────────

/// The partial function call delta within a streaming tool call.
#[pyclass(frozen, name = "StreamFunctionCall")]
pub struct PyStreamFunctionCall {
    inner: liter_llm::types::StreamFunctionCall,
}

#[pymethods]
impl PyStreamFunctionCall {
    /// Partial function name, or `None` if not present in this chunk.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    /// Partial arguments JSON string delta, or `None` if not present in this chunk.
    #[getter]
    fn arguments(&self) -> Option<&str> {
        self.inner.arguments.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "StreamFunctionCall(name={:?}, arguments={:?})",
            self.inner.name, self.inner.arguments
        )
    }

    fn __eq__(&self, other: &PyStreamFunctionCall) -> bool {
        self.inner.name == other.inner.name && self.inner.arguments == other.inner.arguments
    }
}

/// A tool call delta within a streaming chunk.
#[pyclass(frozen, name = "StreamToolCall")]
pub struct PyStreamToolCall {
    inner: liter_llm::types::StreamToolCall,
}

#[pymethods]
impl PyStreamToolCall {
    /// The index of this tool call in the list.
    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }

    /// The tool call ID, present only in the first chunk for this index.
    #[getter]
    fn id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }

    /// The partial function call delta, or `None` if not present in this chunk.
    #[getter]
    fn function(&self) -> Option<PyStreamFunctionCall> {
        self.inner.function.clone().map(|f| PyStreamFunctionCall { inner: f })
    }

    fn __repr__(&self) -> String {
        format!("StreamToolCall(index={}, id={:?})", self.inner.index, self.inner.id)
    }

    fn __eq__(&self, other: &PyStreamToolCall) -> bool {
        self.inner.index == other.inner.index
            && self.inner.id == other.inner.id
            && self.inner.function == other.inner.function
    }
}

/// The delta object in a streaming chunk choice.
#[pyclass(frozen, name = "StreamDelta")]
pub struct PyStreamDelta {
    inner: liter_llm::types::StreamDelta,
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

    /// Refusal text delta, or `None`.
    #[getter]
    fn refusal(&self) -> Option<&str> {
        self.inner.refusal.as_deref()
    }

    /// Tool call deltas, or `None` if not present in this chunk.
    #[getter]
    fn tool_calls(&self) -> Option<Vec<PyStreamToolCall>> {
        self.inner
            .tool_calls
            .as_ref()
            .map(|calls| calls.iter().map(|tc| PyStreamToolCall { inner: tc.clone() }).collect())
    }

    fn __repr__(&self) -> String {
        format!(
            "StreamDelta(role={:?}, content={:?}, refusal={:?})",
            self.inner.role, self.inner.content, self.inner.refusal
        )
    }

    fn __eq__(&self, other: &PyStreamDelta) -> bool {
        self.inner.role == other.inner.role
            && self.inner.content == other.inner.content
            && self.inner.refusal == other.inner.refusal
            && self.inner.tool_calls == other.inner.tool_calls
    }
}

/// A single choice within a streaming chunk.
#[pyclass(frozen, name = "StreamChoice")]
pub struct PyStreamChoice {
    inner: liter_llm::types::StreamChoice,
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
        self.inner.finish_reason.as_ref().map(finish_reason_str)
    }

    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }

    fn __repr__(&self) -> String {
        format!(
            "StreamChoice(index={}, finish_reason={:?})",
            self.inner.index,
            self.finish_reason()
        )
    }

    fn __eq__(&self, other: &PyStreamChoice) -> bool {
        self.inner.index == other.inner.index
            && self.inner.finish_reason == other.inner.finish_reason
            && self.inner.delta == other.inner.delta
    }
}

/// One SSE event from a streaming chat completion.
#[pyclass(frozen, name = "ChatCompletionChunk")]
pub struct PyChatCompletionChunk {
    inner: liter_llm::types::ChatCompletionChunk,
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

    /// Unix timestamp of when this chunk was created.
    #[getter]
    fn created(&self) -> u64 {
        self.inner.created
    }

    #[getter]
    fn choices(&self) -> Vec<PyStreamChoice> {
        self.inner
            .choices
            .iter()
            .map(|c| PyStreamChoice { inner: c.clone() })
            .collect()
    }

    /// Usage statistics, present only in the final chunk when
    /// ``stream_options.include_usage`` was set in the request.
    ///
    /// Note: accessing this getter clones the `Usage` struct (three `u64`
    /// fields) — negligible cost, but documented here for completeness.
    #[getter]
    fn usage(&self) -> Option<PyUsage> {
        self.inner.usage.clone().map(PyUsage::from)
    }

    fn __repr__(&self) -> String {
        format!(
            "ChatCompletionChunk(id={:?}, model={:?}, created={}, choices={})",
            self.inner.id,
            self.inner.model,
            self.inner.created,
            self.inner.choices.len()
        )
    }

    fn __eq__(&self, other: &PyChatCompletionChunk) -> bool {
        self.inner.id == other.inner.id
            && self.inner.model == other.inner.model
            && self.inner.created == other.inner.created
            && self.inner.choices == other.inner.choices
    }
}

impl From<liter_llm::types::ChatCompletionChunk> for PyChatCompletionChunk {
    fn from(inner: liter_llm::types::ChatCompletionChunk) -> Self {
        Self { inner }
    }
}

// ─── Embedding types ──────────────────────────────────────────────────────────

/// A single embedding vector.
///
/// Note: the `.embedding` getter clones the entire `Vec<f64>` into a Python
/// list.  For large vectors consider slicing with indexing instead.
#[pyclass(frozen, name = "EmbeddingObject")]
pub struct PyEmbeddingObject {
    inner: liter_llm::types::EmbeddingObject,
}

#[pymethods]
impl PyEmbeddingObject {
    /// The embedding vector.
    ///
    /// This clones the full vector on each access.  Cache the result in Python
    /// when you need to access it multiple times.
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

    fn __eq__(&self, other: &PyEmbeddingObject) -> bool {
        self.inner.index == other.inner.index && self.inner.embedding == other.inner.embedding
    }
}

/// Response from an embedding request.
#[pyclass(frozen, name = "EmbeddingResponse")]
pub struct PyEmbeddingResponse {
    inner: liter_llm::types::EmbeddingResponse,
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

    /// Token usage for this request.  Returns `None` when the provider does not
    /// include usage data in the response.
    #[getter]
    fn usage(&self) -> Option<PyUsage> {
        self.inner.usage.clone().map(|u| PyUsage { inner: u })
    }

    fn __repr__(&self) -> String {
        format!(
            "EmbeddingResponse(model={:?}, count={})",
            self.inner.model,
            self.inner.data.len()
        )
    }

    fn __eq__(&self, other: &PyEmbeddingResponse) -> bool {
        self.inner.model == other.inner.model
            && self.inner.data == other.inner.data
            && self.inner.usage == other.inner.usage
    }
}

impl From<liter_llm::types::EmbeddingResponse> for PyEmbeddingResponse {
    fn from(inner: liter_llm::types::EmbeddingResponse) -> Self {
        Self { inner }
    }
}

// ─── Models list types ────────────────────────────────────────────────────────

/// Information about a single model.
#[pyclass(frozen, name = "ModelObject")]
pub struct PyModelObject {
    inner: liter_llm::types::ModelObject,
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
        format!(
            "ModelObject(id={:?}, owned_by={:?}, created={})",
            self.inner.id, self.inner.owned_by, self.inner.created
        )
    }

    fn __eq__(&self, other: &PyModelObject) -> bool {
        self.inner.id == other.inner.id
            && self.inner.owned_by == other.inner.owned_by
            && self.inner.created == other.inner.created
            && self.inner.object == other.inner.object
    }
}

/// Response from a list-models request.
#[pyclass(frozen, name = "ModelsListResponse")]
pub struct PyModelsListResponse {
    inner: liter_llm::types::ModelsListResponse,
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

    fn __eq__(&self, other: &PyModelsListResponse) -> bool {
        self.inner.data == other.inner.data
    }
}

impl From<liter_llm::types::ModelsListResponse> for PyModelsListResponse {
    fn from(inner: liter_llm::types::ModelsListResponse) -> Self {
        Self { inner }
    }
}

// ─── Image generation types ──────────────────────────────────────────────────

/// A single generated image, returned as either a URL or base64 data.
#[pyclass(frozen, skip_from_py_object, name = "Image")]
#[derive(Clone)]
pub struct PyImage {
    inner: liter_llm::types::Image,
}

#[pymethods]
impl PyImage {
    /// URL of the generated image, or `None` if `b64_json` was requested.
    #[getter]
    fn url(&self) -> Option<&str> {
        self.inner.url.as_deref()
    }

    /// Base64-encoded image data, or `None` if `url` format was requested.
    #[getter]
    fn b64_json(&self) -> Option<&str> {
        self.inner.b64_json.as_deref()
    }

    /// The prompt that was used to generate this image, revised by the model.
    #[getter]
    fn revised_prompt(&self) -> Option<&str> {
        self.inner.revised_prompt.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "Image(url={:?}, b64_json={}, revised_prompt={:?})",
            self.inner.url,
            self.inner.b64_json.as_ref().map_or("None", |_| "..."),
            self.inner.revised_prompt
        )
    }

    fn __eq__(&self, other: &PyImage) -> bool {
        self.inner == other.inner
    }
}

/// Response containing generated images.
#[pyclass(frozen, name = "ImagesResponse")]
pub struct PyImagesResponse {
    inner: liter_llm::types::ImagesResponse,
}

#[pymethods]
impl PyImagesResponse {
    /// Unix timestamp of when the images were created.
    #[getter]
    fn created(&self) -> u64 {
        self.inner.created
    }

    /// List of generated images.
    #[getter]
    fn data(&self) -> Vec<PyImage> {
        self.inner
            .data
            .iter()
            .map(|img| PyImage { inner: img.clone() })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "ImagesResponse(created={}, count={})",
            self.inner.created,
            self.inner.data.len()
        )
    }

    fn __eq__(&self, other: &PyImagesResponse) -> bool {
        self.inner == other.inner
    }
}

impl From<liter_llm::types::ImagesResponse> for PyImagesResponse {
    fn from(inner: liter_llm::types::ImagesResponse) -> Self {
        Self { inner }
    }
}

// ─── Transcription types ─────────────────────────────────────────────────────

/// A segment of transcribed audio with timing information.
#[pyclass(frozen, skip_from_py_object, name = "TranscriptionSegment")]
#[derive(Clone)]
pub struct PyTranscriptionSegment {
    inner: liter_llm::types::TranscriptionSegment,
}

#[pymethods]
impl PyTranscriptionSegment {
    #[getter]
    fn id(&self) -> u32 {
        self.inner.id
    }

    #[getter]
    fn start(&self) -> f64 {
        self.inner.start
    }

    #[getter]
    fn end(&self) -> f64 {
        self.inner.end
    }

    #[getter]
    fn text(&self) -> &str {
        &self.inner.text
    }

    fn __repr__(&self) -> String {
        format!(
            "TranscriptionSegment(id={}, start={}, end={}, text={:?})",
            self.inner.id, self.inner.start, self.inner.end, self.inner.text
        )
    }

    fn __eq__(&self, other: &PyTranscriptionSegment) -> bool {
        self.inner == other.inner
    }
}

/// Response from a transcription request.
#[pyclass(frozen, name = "TranscriptionResponse")]
pub struct PyTranscriptionResponse {
    inner: liter_llm::types::TranscriptionResponse,
}

#[pymethods]
impl PyTranscriptionResponse {
    /// The transcribed text.
    #[getter]
    fn text(&self) -> &str {
        &self.inner.text
    }

    /// The detected language, or `None`.
    #[getter]
    fn language(&self) -> Option<&str> {
        self.inner.language.as_deref()
    }

    /// Duration of the audio in seconds, or `None`.
    #[getter]
    fn duration(&self) -> Option<f64> {
        self.inner.duration
    }

    /// Transcription segments with timing, or `None`.
    #[getter]
    fn segments(&self) -> Option<Vec<PyTranscriptionSegment>> {
        self.inner.segments.as_ref().map(|segs| {
            segs.iter()
                .map(|s| PyTranscriptionSegment { inner: s.clone() })
                .collect()
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "TranscriptionResponse(text={:?}, language={:?}, duration={:?})",
            self.inner.text, self.inner.language, self.inner.duration
        )
    }

    fn __eq__(&self, other: &PyTranscriptionResponse) -> bool {
        self.inner == other.inner
    }
}

impl From<liter_llm::types::TranscriptionResponse> for PyTranscriptionResponse {
    fn from(inner: liter_llm::types::TranscriptionResponse) -> Self {
        Self { inner }
    }
}

// ─── Moderation types ────────────────────────────────────────────────────────

/// Boolean flags for each moderation category.
#[pyclass(frozen, skip_from_py_object, name = "ModerationCategories")]
#[derive(Clone)]
pub struct PyModerationCategories {
    inner: liter_llm::types::ModerationCategories,
}

#[pymethods]
impl PyModerationCategories {
    #[getter]
    fn sexual(&self) -> bool {
        self.inner.sexual
    }
    #[getter]
    fn hate(&self) -> bool {
        self.inner.hate
    }
    #[getter]
    fn harassment(&self) -> bool {
        self.inner.harassment
    }
    #[getter]
    fn self_harm(&self) -> bool {
        self.inner.self_harm
    }
    #[getter]
    fn sexual_minors(&self) -> bool {
        self.inner.sexual_minors
    }
    #[getter]
    fn hate_threatening(&self) -> bool {
        self.inner.hate_threatening
    }
    #[getter]
    fn violence_graphic(&self) -> bool {
        self.inner.violence_graphic
    }
    #[getter]
    fn self_harm_intent(&self) -> bool {
        self.inner.self_harm_intent
    }
    #[getter]
    fn self_harm_instructions(&self) -> bool {
        self.inner.self_harm_instructions
    }
    #[getter]
    fn harassment_threatening(&self) -> bool {
        self.inner.harassment_threatening
    }
    #[getter]
    fn violence(&self) -> bool {
        self.inner.violence
    }

    fn __repr__(&self) -> String {
        format!(
            "ModerationCategories(sexual={}, hate={}, harassment={}, violence={})",
            self.inner.sexual, self.inner.hate, self.inner.harassment, self.inner.violence
        )
    }

    fn __eq__(&self, other: &PyModerationCategories) -> bool {
        self.inner == other.inner
    }
}

/// Confidence scores for each moderation category.
#[pyclass(frozen, skip_from_py_object, name = "ModerationCategoryScores")]
#[derive(Clone)]
pub struct PyModerationCategoryScores {
    inner: liter_llm::types::ModerationCategoryScores,
}

#[pymethods]
impl PyModerationCategoryScores {
    #[getter]
    fn sexual(&self) -> f64 {
        self.inner.sexual
    }
    #[getter]
    fn hate(&self) -> f64 {
        self.inner.hate
    }
    #[getter]
    fn harassment(&self) -> f64 {
        self.inner.harassment
    }
    #[getter]
    fn self_harm(&self) -> f64 {
        self.inner.self_harm
    }
    #[getter]
    fn sexual_minors(&self) -> f64 {
        self.inner.sexual_minors
    }
    #[getter]
    fn hate_threatening(&self) -> f64 {
        self.inner.hate_threatening
    }
    #[getter]
    fn violence_graphic(&self) -> f64 {
        self.inner.violence_graphic
    }
    #[getter]
    fn self_harm_intent(&self) -> f64 {
        self.inner.self_harm_intent
    }
    #[getter]
    fn self_harm_instructions(&self) -> f64 {
        self.inner.self_harm_instructions
    }
    #[getter]
    fn harassment_threatening(&self) -> f64 {
        self.inner.harassment_threatening
    }
    #[getter]
    fn violence(&self) -> f64 {
        self.inner.violence
    }

    fn __repr__(&self) -> String {
        format!(
            "ModerationCategoryScores(sexual={}, hate={}, harassment={}, violence={})",
            self.inner.sexual, self.inner.hate, self.inner.harassment, self.inner.violence
        )
    }

    fn __eq__(&self, other: &PyModerationCategoryScores) -> bool {
        self.inner == other.inner
    }
}

/// A single moderation classification result.
#[pyclass(frozen, skip_from_py_object, name = "ModerationResult")]
#[derive(Clone)]
pub struct PyModerationResult {
    inner: liter_llm::types::ModerationResult,
}

#[pymethods]
impl PyModerationResult {
    /// Whether the content was flagged by any category.
    #[getter]
    fn flagged(&self) -> bool {
        self.inner.flagged
    }

    /// Boolean flags for each moderation category.
    #[getter]
    fn categories(&self) -> PyModerationCategories {
        PyModerationCategories {
            inner: self.inner.categories.clone(),
        }
    }

    /// Confidence scores for each moderation category.
    #[getter]
    fn category_scores(&self) -> PyModerationCategoryScores {
        PyModerationCategoryScores {
            inner: self.inner.category_scores.clone(),
        }
    }

    fn __repr__(&self) -> String {
        format!("ModerationResult(flagged={})", self.inner.flagged)
    }

    fn __eq__(&self, other: &PyModerationResult) -> bool {
        self.inner == other.inner
    }
}

/// Response from the moderation endpoint.
#[pyclass(frozen, name = "ModerationResponse")]
pub struct PyModerationResponse {
    inner: liter_llm::types::ModerationResponse,
}

#[pymethods]
impl PyModerationResponse {
    /// Unique identifier for this moderation request.
    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    /// The model used for moderation.
    #[getter]
    fn model(&self) -> &str {
        &self.inner.model
    }

    /// List of moderation results, one per input.
    #[getter]
    fn results(&self) -> Vec<PyModerationResult> {
        self.inner
            .results
            .iter()
            .map(|r| PyModerationResult { inner: r.clone() })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "ModerationResponse(id={:?}, model={:?}, results={})",
            self.inner.id,
            self.inner.model,
            self.inner.results.len()
        )
    }

    fn __eq__(&self, other: &PyModerationResponse) -> bool {
        self.inner == other.inner
    }
}

impl From<liter_llm::types::ModerationResponse> for PyModerationResponse {
    fn from(inner: liter_llm::types::ModerationResponse) -> Self {
        Self { inner }
    }
}

// ─── Rerank types ────────────────────────────────────────────────────────────

/// The text content of a reranked document.
#[pyclass(frozen, skip_from_py_object, name = "RerankResultDocument")]
#[derive(Clone)]
pub struct PyRerankResultDocument {
    inner: liter_llm::types::RerankResultDocument,
}

#[pymethods]
impl PyRerankResultDocument {
    #[getter]
    fn text(&self) -> &str {
        &self.inner.text
    }

    fn __repr__(&self) -> String {
        format!("RerankResultDocument(text={:?})", self.inner.text)
    }

    fn __eq__(&self, other: &PyRerankResultDocument) -> bool {
        self.inner == other.inner
    }
}

/// A single reranked document with its relevance score.
#[pyclass(frozen, skip_from_py_object, name = "RerankResult")]
#[derive(Clone)]
pub struct PyRerankResult {
    inner: liter_llm::types::RerankResult,
}

#[pymethods]
impl PyRerankResult {
    /// Original index of the document in the input list.
    #[getter]
    fn index(&self) -> u32 {
        self.inner.index
    }

    /// Relevance score assigned by the reranker.
    #[getter]
    fn relevance_score(&self) -> f64 {
        self.inner.relevance_score
    }

    /// The document content, present when ``return_documents`` was set.
    #[getter]
    fn document(&self) -> Option<PyRerankResultDocument> {
        self.inner
            .document
            .as_ref()
            .map(|d| PyRerankResultDocument { inner: d.clone() })
    }

    fn __repr__(&self) -> String {
        format!(
            "RerankResult(index={}, relevance_score={})",
            self.inner.index, self.inner.relevance_score
        )
    }

    fn __eq__(&self, other: &PyRerankResult) -> bool {
        self.inner == other.inner
    }
}

/// Response from the rerank endpoint.
#[pyclass(frozen, name = "RerankResponse")]
pub struct PyRerankResponse {
    inner: liter_llm::types::RerankResponse,
}

#[pymethods]
impl PyRerankResponse {
    /// Unique identifier for this rerank request, or `None`.
    #[getter]
    fn id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }

    /// Reranked results sorted by relevance.
    #[getter]
    fn results(&self) -> Vec<PyRerankResult> {
        self.inner
            .results
            .iter()
            .map(|r| PyRerankResult { inner: r.clone() })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "RerankResponse(id={:?}, results={})",
            self.inner.id,
            self.inner.results.len()
        )
    }

    fn __eq__(&self, other: &PyRerankResponse) -> bool {
        self.inner == other.inner
    }
}

impl From<liter_llm::types::RerankResponse> for PyRerankResponse {
    fn from(inner: liter_llm::types::RerankResponse) -> Self {
        Self { inner }
    }
}

// ─── JSON-to-Python conversion helper ────────────────────────────────────────

/// A wrapper around `serde_json::Value` that converts to native Python objects
/// when returned from async methods.
///
/// This implements `IntoPyObject` so it can be returned directly from
/// `future_into_py` closures.
pub struct JsonValue(pub serde_json::Value);

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        json_to_py_bound(py, &self.0)
    }
}

/// Convert a `serde_json::Value` into a native Python object (dict, list, str,
/// int, float, bool, or None).  Used for management API responses that return
/// plain dicts rather than typed pyclasses.
fn json_to_py_bound<'py>(py: Python<'py>, val: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
    match val {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.to_owned().into_any())
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.to_owned().into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any())
            } else {
                Ok(py.None().into_bound(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any()),
        serde_json::Value::Array(arr) => {
            let list = pyo3::types::PyList::empty(py);
            for item in arr {
                list.append(json_to_py_bound(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(map) => {
            let dict = pyo3::types::PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py_bound(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

/// Serialize a value to a `JsonValue` wrapper suitable for returning from async
/// methods.  The wrapper implements `IntoPyObject` for automatic conversion.
pub fn to_json_value<T: serde::Serialize>(val: &T) -> PyResult<JsonValue> {
    let json = serde_json::to_value(val).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(JsonValue(json))
}

/// Register all response types on the module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUsage>()?;
    m.add_class::<PyFunctionCall>()?;
    m.add_class::<PyToolCall>()?;
    m.add_class::<PyAssistantMessage>()?;
    m.add_class::<PyChoice>()?;
    m.add_class::<PyChatCompletionResponse>()?;
    m.add_class::<PyStreamFunctionCall>()?;
    m.add_class::<PyStreamToolCall>()?;
    m.add_class::<PyStreamDelta>()?;
    m.add_class::<PyStreamChoice>()?;
    m.add_class::<PyChatCompletionChunk>()?;
    m.add_class::<PyEmbeddingObject>()?;
    m.add_class::<PyEmbeddingResponse>()?;
    m.add_class::<PyModelObject>()?;
    m.add_class::<PyModelsListResponse>()?;
    // Image generation types
    m.add_class::<PyImage>()?;
    m.add_class::<PyImagesResponse>()?;
    // Transcription types
    m.add_class::<PyTranscriptionSegment>()?;
    m.add_class::<PyTranscriptionResponse>()?;
    // Moderation types
    m.add_class::<PyModerationCategories>()?;
    m.add_class::<PyModerationCategoryScores>()?;
    m.add_class::<PyModerationResult>()?;
    m.add_class::<PyModerationResponse>()?;
    // Rerank types
    m.add_class::<PyRerankResultDocument>()?;
    m.add_class::<PyRerankResult>()?;
    m.add_class::<PyRerankResponse>()?;
    Ok(())
}
