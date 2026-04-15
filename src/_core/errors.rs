/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyArithmeticError;
use pyo3::exceptions::PyException;
use pyo3::prelude::PyErr;
use std::fmt::Debug;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn err_to_string<E>(err: E) -> String
where
    E: Debug,
{
    format!("{err:?}")
}
