use pyo3::prelude::*;

mod client;
mod error;
mod types;

/// Python bindings for liter-lm.
#[pymodule]
fn _internal_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register exception hierarchy first (subclasses reference parents).
    error::register(m)?;

    // Register response types.
    types::register(m)?;

    // Register the main client class.
    m.add_class::<client::PyLlmClient>()?;
    m.add_class::<client::PyAsyncChunkIterator>()?;

    Ok(())
}
