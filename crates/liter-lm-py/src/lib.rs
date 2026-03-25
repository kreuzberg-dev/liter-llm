use pyo3::prelude::*;

/// Python bindings for liter-lm.
/// Functions and classes will be added here as the API stabilizes.
#[pymodule]
fn _internal_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
