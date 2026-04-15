/// Module containing python bindings

/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use pyo3::Bound;
use pyo3::Python;
use pyo3::prelude::PyResult;
use pyo3::prelude::pymodule;
use pyo3::types::PyModule;
use pyo3::types::PyModuleMethods;
use pyo3::prelude::wrap_pyfunction;

pub mod errors;
pub mod hello;
pub mod json;
pub mod search;
pub mod bim;

pub use errors::err_to_py_exception;
pub use errors::err_to_py_exception_arithmetic;
pub use errors::err_to_py_exception_type;
pub use errors::err_to_string;

/// ----------------------------------------------------------------
/// BINDINGS
/// ----------------------------------------------------------------

#[pymodule(name = "example_package")]
pub fn createmodule<'a>(_py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(hello::greet, m)?)?;
    m.add_function(wrap_pyfunction!(search::binary_search, m)?)?;
    m.add_class::<bim::PsetId>()?;
    m.add_class::<bim::Pset>()?;
    m.add_class::<bim::Psets>()?;
    return Ok(());
}
