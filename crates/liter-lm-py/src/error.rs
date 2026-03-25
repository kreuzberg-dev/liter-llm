use liter_lm::LiterLmError;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Base exception
create_exception!(
    _internal_bindings,
    LlmError,
    PyException,
    "Base exception for all liter-lm errors."
);

// Specific error subclasses
create_exception!(
    _internal_bindings,
    AuthenticationError,
    LlmError,
    "HTTP 401 – invalid or missing API key."
);
create_exception!(
    _internal_bindings,
    RateLimitedError,
    LlmError,
    "HTTP 429 – rate limit exceeded."
);
create_exception!(
    _internal_bindings,
    ServerError,
    LlmError,
    "HTTP 5xx – upstream server error."
);
create_exception!(
    _internal_bindings,
    NotFoundError,
    LlmError,
    "HTTP 404 – resource not found."
);
create_exception!(
    _internal_bindings,
    ServiceUnavailableError,
    LlmError,
    "HTTP 502/503 – service unavailable."
);
create_exception!(_internal_bindings, BadRequestError, LlmError, "HTTP 400 – bad request.");
create_exception!(
    _internal_bindings,
    ContextWindowExceededError,
    BadRequestError,
    "HTTP 400 – prompt exceeds the model's context window."
);
create_exception!(
    _internal_bindings,
    ContentPolicyError,
    BadRequestError,
    "HTTP 400 – request was rejected by the provider's content policy."
);
create_exception!(
    _internal_bindings,
    LlmTimeoutError,
    LlmError,
    "Request timed out before the provider responded."
);
create_exception!(
    _internal_bindings,
    NetworkError,
    LlmError,
    "Network-level error communicating with the provider."
);
create_exception!(
    _internal_bindings,
    StreamingError,
    LlmError,
    "Error while reading a streaming response from the provider."
);
create_exception!(
    _internal_bindings,
    InvalidHeaderError,
    LlmError,
    "An HTTP header name or value was invalid."
);
create_exception!(
    _internal_bindings,
    EndpointNotSupportedError,
    LlmError,
    "The requested endpoint is not supported by the selected provider."
);

/// Convert a [`LiterLmError`] into the matching Python exception.
pub fn to_py_err(e: LiterLmError) -> PyErr {
    let msg = e.to_string();
    match e {
        LiterLmError::Authentication { .. } => PyErr::new::<AuthenticationError, _>(msg),
        LiterLmError::RateLimited { .. } => PyErr::new::<RateLimitedError, _>(msg),
        LiterLmError::ServerError { .. } => PyErr::new::<ServerError, _>(msg),
        LiterLmError::NotFound { .. } => PyErr::new::<NotFoundError, _>(msg),
        LiterLmError::ServiceUnavailable { .. } => PyErr::new::<ServiceUnavailableError, _>(msg),
        LiterLmError::BadRequest { .. } => PyErr::new::<BadRequestError, _>(msg),
        LiterLmError::ContextWindowExceeded { .. } => PyErr::new::<ContextWindowExceededError, _>(msg),
        LiterLmError::ContentPolicy { .. } => PyErr::new::<ContentPolicyError, _>(msg),
        LiterLmError::Timeout => PyErr::new::<LlmTimeoutError, _>(msg),
        LiterLmError::Network(_) => PyErr::new::<NetworkError, _>(msg),
        LiterLmError::Streaming { .. } => PyErr::new::<StreamingError, _>(msg),
        LiterLmError::InvalidHeader { .. } => PyErr::new::<InvalidHeaderError, _>(msg),
        LiterLmError::EndpointNotSupported { .. } => PyErr::new::<EndpointNotSupportedError, _>(msg),
        _ => PyErr::new::<LlmError, _>(msg),
    }
}

/// Register all exception types on the module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("LlmError", m.py().get_type::<LlmError>())?;
    m.add("AuthenticationError", m.py().get_type::<AuthenticationError>())?;
    m.add("RateLimitedError", m.py().get_type::<RateLimitedError>())?;
    m.add("ServerError", m.py().get_type::<ServerError>())?;
    m.add("NotFoundError", m.py().get_type::<NotFoundError>())?;
    m.add("ServiceUnavailableError", m.py().get_type::<ServiceUnavailableError>())?;
    m.add("BadRequestError", m.py().get_type::<BadRequestError>())?;
    m.add(
        "ContextWindowExceededError",
        m.py().get_type::<ContextWindowExceededError>(),
    )?;
    m.add("ContentPolicyError", m.py().get_type::<ContentPolicyError>())?;
    m.add("LlmTimeoutError", m.py().get_type::<LlmTimeoutError>())?;
    m.add("NetworkError", m.py().get_type::<NetworkError>())?;
    m.add("StreamingError", m.py().get_type::<StreamingError>())?;
    m.add("InvalidHeaderError", m.py().get_type::<InvalidHeaderError>())?;
    m.add(
        "EndpointNotSupportedError",
        m.py().get_type::<EndpointNotSupportedError>(),
    )?;
    Ok(())
}
