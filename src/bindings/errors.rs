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
    format!("{:?}", err)
}

/// Convert an error to a python Exception
pub fn err_to_py_exception<E>(err: E) -> PyErr
where
    E: Debug,
{
    let err_str = format!("{err:?}");
    return PyErr::new::<PyException, _>(err_str);
}

/// Convert an error to a python ArithmeticException
pub fn err_to_py_exception_arithmetic<E>(err: E) -> PyErr
where
    E: Debug,
{
    let err_str = format!("{err:?}");
    return PyErr::new::<PyArithmeticError, _>(err_str);
}

/// Convert an error to a python TypeError
pub fn err_to_py_exception_type<E>(err: E) -> PyErr
where
    E: Debug,
{
    let err_str = format!("{err:?}");
    return PyErr::new::<PyTypeError, _>(err_str);
}
