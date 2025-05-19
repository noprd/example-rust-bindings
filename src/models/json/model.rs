// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use pyo3::Bound;
use pyo3::FromPyObject;
use pyo3::IntoPyObject;
use pyo3::IntoPyObjectExt;
use pyo3::PyErr;
use pyo3::Python;
use pyo3::types::PyAny;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::types::PyDictMethods;
use pyo3::types::PyList;
use pyo3::types::PyListMethods;
use pyo3::types::PyString;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use std::result::Result;

use super::base::JsonConversion;
use crate::_core::errors::err_to_py_string;

// ----------------------------------------------------------------
// STRUCTURES/TYPES
// ----------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueWrap(pub Value);

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF ValueWrate
// ----------------------------------------------------------------

impl<'a> ValueWrap {
    pub fn to_python(self, py: Python<'a>) -> Result<Bound<'a, PyAny>, PyErr> {
        self.into_pyobject(py)
    }
}

impl JsonConversion<Value> for ValueWrap {
    fn from_json(value: &Value) -> std::result::Result<Self, String> {
        let result = ValueWrap(value.clone());
        return Ok(result);
    }

    fn to_json(&self) -> std::result::Result<Value, String> {
        let ValueWrap(value) = self;
        return Ok(value.clone());
    }
}

impl<'a> FromPyObject<'a> for ValueWrap {
    fn extract_bound(value: &Bound<'a, PyAny>) -> Result<Self, PyErr> {
        if value.is_none() {
            let result = ValueWrap(Value::Null);
            return Ok(result);
        } else if let Ok(val) = value.extract::<bool>() {
            let result = ValueWrap(Value::Bool(val));
            return Ok(result);
        } else if let Ok(val) = value.extract::<i64>() {
            let result = ValueWrap(Value::Number(Number::from(val)));
            return Ok(result);
        } else if let Ok(val) = value.extract::<f64>() {
            let result = ValueWrap(Value::Number(Number::from_f64(val).unwrap()));
            return Ok(result);
        } else if let Ok(val) = value.extract::<String>() {
            let result = ValueWrap(Value::String(val));
            return Ok(result);
        } else if let Ok(items) = value.extract::<Bound<'a, PyList>>() {
            let elements: Vec<Value> = items
                .iter()
                .map(|x| ValueWrap::extract_bound(&x).unwrap())
                .map(|ValueWrap(x)| x)
                .collect();
            let result = ValueWrap(Value::Array(elements));
            return Ok(result);
        } else if let Ok(items) = value.extract::<Bound<'a, PyDict>>() {
            let elements: Map<String, Value> = items
                .into_iter()
                .map(|(x, y)| {
                    let key: String = x.extract().unwrap();
                    let value = ValueWrap::extract_bound(&y).unwrap();
                    return (key, value);
                })
                .map(|(x, ValueWrap(y))| (x, y))
                .collect();
            let result = ValueWrap(Value::Object(elements));
            return Ok(result);
        } else {
            return Err(err_to_py_string("Invalid type"));
        }
    }
}

impl<'a> IntoPyObject<'a> for ValueWrap {
    type Target = PyAny;
    type Output = Bound<'a, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'a>) -> Result<Self::Output, Self::Error> {
        let ValueWrap(value) = self;
        match value {
            Value::Null => {
                let result = py.None().into_bound(py);
                return Ok(result);
            }
            Value::Bool(x) => {
                let result = x.into_bound_py_any(py)?;
                return Ok(result);
            }
            Value::String(s) => {
                let result = PyString::new(py, &s).into_bound_py_any(py)?;
                return Ok(result);
            }
            Value::Number(n) => {
                if let Some(x) = n.as_u64() {
                    let result = x.into_bound_py_any(py)?;
                    return Ok(result);
                }
                if let Some(x) = n.as_i64() {
                    let result = x.into_bound_py_any(py)?;
                    return Ok(result);
                }
                if let Some(x) = n.as_f64() {
                    let result = x.into_bound_py_any(py)?;
                    return Ok(result);
                }
                return Err(err_to_py_string("Invalid numerical type"));
            }
            Value::Array(items) => {
                let elements: Vec<Self::Output> = items
                    .iter()
                    .map(|x| ValueWrap(x.clone()))
                    .map(|x| x.into_pyobject(py).unwrap())
                    .collect();
                let result = PyList::new(py, elements)?.into_any();
                return Ok(result);
            }
            Value::Object(items) => {
                let elements = PyDict::new(py);
                for (key, x) in items.iter() {
                    let value = ValueWrap(x.clone()).into_pyobject(py)?;
                    elements.set_item(key, value)?;
                }
                let result = elements.into_any();
                return Ok(result);
            }
        }
    }
}
