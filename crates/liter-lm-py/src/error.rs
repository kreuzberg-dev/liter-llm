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

/// Convert a [`LiterLmError`] into the matching Python exception.
pub fn to_py_err(e: LiterLmError) -> PyErr {
    let msg = e.to_string();
    match e {
        LiterLmError::Authentication { .. } => PyErr::new::<AuthenticationError, _>(msg),
        LiterLmError::RateLimited { .. } => PyErr::new::<RateLimitedError, _>(msg),
        LiterLmError::ServerError { .. } => PyErr::new::<ServerError, _>(msg),
        LiterLmError::NotFound { .. } => PyErr::new::<NotFoundError, _>(msg),
        LiterLmError::ServiceUnavailable { .. } => PyErr::new::<ServiceUnavailableError, _>(msg),
        LiterLmError::BadRequest { .. }
        | LiterLmError::ContextWindowExceeded { .. }
        | LiterLmError::ContentPolicy { .. } => PyErr::new::<BadRequestError, _>(msg),
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
    Ok(())
}
