use pyo3::prelude::*;

/// Add two unsigned integers together.
#[pyfunction]
fn add(first: u32, second: u32) -> PyResult<u32> {
    Ok(cdir_core::add(first, second))
}

/// A Python module implemented in Rust.
#[pymodule]
fn cdir_python(_py: Python, m: &PyModule) -> PyResult<()> {
    let wrapped = pyo3::wrap_pyfunction!(add, m)?;
    m.add_function(wrapped)?;

    Ok(())
}
