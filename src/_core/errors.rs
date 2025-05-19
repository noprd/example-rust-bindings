// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::PyErr;
use std::fmt::Debug;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

pub fn err_to_string<E>(err: E) -> String
where
    E: Debug,
{
    format!("{:?}", err)
}

pub fn err_to_py_string<E>(err: E) -> PyErr
where
    E: Debug,
{
    let err_str = format!("{:?}", err);
    return PyErr::new::<PyTypeError, _>(err_str);
}
