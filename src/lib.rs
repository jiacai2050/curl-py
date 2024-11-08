use curl_sys as sys;
use pyo3::prelude::*;

#[pyfunction]
fn curl_get(url: String) -> PyResult<String> {
    Ok(sys::curl_get(url))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn curl_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(curl_get, m)?)?;
    Ok(())
}
