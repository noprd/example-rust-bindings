// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use pyo3::Bound;
use pyo3::Python;
use pyo3::prelude::PyResult;
use pyo3::prelude::pymodule;
use pyo3::types::PyModule;
use pyo3::types::PyModuleMethods;
// use pyo3::FromPyObject;
// use pyo3::exceptions::PyTypeError;
// use pyo3::prelude::PyErr;
// use pyo3::prelude::pyfunction;
// use pyo3::prelude::wrap_pyfunction;
// use pyo3::types::PyAny;

mod _core;
mod models;
use models::bim;
// use models::tree;
// use models::json;

// ----------------------------------------------------------------
// MAIN EXPORTER
// ----------------------------------------------------------------

#[pymodule(name = "example_package")]
pub fn createmodule<'a>(_py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(probe, m)?)?;
    m.add_class::<bim::psets::PsetId>()?;
    m.add_class::<bim::psets::Pset>()?;
    m.add_class::<bim::psets::Psets>()?;
    return Ok(());
}

// #[pyfunction]
// fn probe<'a>(py: Python<'a>, object: &Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
//     let value = json::ValueWrap::extract_bound(object)?;
//     let obj = value
//         .to_python(py)
//         .map_err(_core::errors::err_to_py_string)?;
//     return Ok(obj);
// }
