use curl_sys as sys;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (url, redirect = true, verbose = false))]
fn get(url: &str, redirect: bool, verbose: bool) -> PyResult<String> {
    let res = sys::curl_get(url, redirect, verbose).map_err(|e| PyValueError::new_err(e))?;
    Ok(res)
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
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}
