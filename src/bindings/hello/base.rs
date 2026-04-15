/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use pyo3::Python;
use pyo3::PyResult;
use pyo3::prelude::pyfunction;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

#[pyfunction]
#[pyo3(name="greet")]
#[pyo3(signature = (name=None, /, *))]
pub fn greet<'py>(
    _ctx: Python<'py>,
    name: Option<String>,
) -> PyResult<String> {
    let msg: String;
    match name {
        Some(name) => {
            msg = format!("Hello, {name}!");
        },
        _ => {
            msg = "Hello world!".to_string();
        }
    }
    return Ok(msg);
}
