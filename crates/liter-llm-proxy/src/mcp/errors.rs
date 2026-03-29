use liter_llm::error::LiterLlmError;
use rmcp::ErrorData;

/// Convert a `LiterLlmError` to an rmcp `ErrorData`.
///
/// Maps client-facing errors (bad request, auth, serialization) to
/// `invalid_params`, and all other errors to `internal_error`.
pub fn to_error_data(e: LiterLlmError) -> ErrorData {
    match e {
        LiterLlmError::BadRequest { .. }
        | LiterLlmError::InvalidHeader { .. }
        | LiterLlmError::Serialization(_)
        | LiterLlmError::ContextWindowExceeded { .. }
        | LiterLlmError::ContentPolicy { .. } => ErrorData::invalid_params(e.to_string(), None),
        LiterLlmError::Authentication { .. } => ErrorData::invalid_params(e.to_string(), None),
        LiterLlmError::NotFound { .. } => ErrorData::resource_not_found(e.to_string(), None),
        _ => ErrorData::internal_error(e.to_string(), None),
    }
}

/// Convert a `String` error to an rmcp `ErrorData`.
pub fn string_to_error_data(e: String) -> ErrorData {
    ErrorData::internal_error(e, None)
}
