/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use pyo3::Python;
use pyo3::PyResult;
use pyo3::prelude::pyfunction;

use crate::algorithms::search;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

#[pyfunction]
#[pyo3(name="binary_search")]
#[pyo3(signature = (*, data, element))]
pub fn binary_search<'py>(
    _ctx: Python<'py>,
    data: Vec<String>,
    element: String,
) -> PyResult<Option<usize>> {
    let result = search::binary_search(&data, &element);
    match result {
        Ok(index) => {
            return Ok(Some(index));
        },
        Err(err) => {
            println!("failed to find '{element}' in array - {err:?}");
            return Ok(None);
        },
    }
}
