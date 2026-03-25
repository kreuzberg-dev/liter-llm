// C FFI bindings for liter-lm.
// Exported functions will be added here as the API stabilizes.

/// Returns the version string of the liter-lm library.
///
/// # Safety
/// The returned pointer is valid for the lifetime of the program and must NOT be freed.
#[unsafe(no_mangle)]
pub extern "C" fn liter_lm_version() -> *const std::ffi::c_char {
    // SAFETY: This string literal is 'static and null-terminated.
    c"0.1.0".as_ptr()
}
