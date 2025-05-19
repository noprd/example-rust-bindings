// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use pyo3::Bound;
use pyo3::FromPyObject; // NOTE: needed, in order to "import" the FromPyObject implementations of other structs
use pyo3::IntoPyObject;
use pyo3::PyRef;
use pyo3::PyRefMut;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::prelude::pyclass;
use pyo3::prelude::pymethods;
use pyo3::types::PyAny;
use pyo3::types::PyTuple;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;
use std::result::Result;

use crate::_core::errors::err_to_py_string;
use crate::_core::errors::err_to_string;
use crate::models::json::base::JsonConversion;
use crate::models::json::model::ValueWrap;
use crate::models::tree::base::GenericTree;

// ----------------------------------------------------------------
// STRUCTS
// ----------------------------------------------------------------

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PsetId {
    #[serde(alias = "id")]
    pub id_: i64,
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pset {
    #[serde(alias = "id")]
    pub id_: i64,
    #[serde(alias = "class")]
    pub class_: String,
    // need this to be able to handle pyo3 traits
    pub value: ValueWrap,
    #[serde(alias = "value-type")]
    pub value_type: Option<String>,
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PsetFlattenedValue {
    Pset(Pset),
    PsetId(PsetId),
    Value(ValueWrap),
}

#[derive(Clone)]
pub struct PsetFlattenedValueWithAddress {
    addr: Option<String>,
    entity: Option<PsetFlattenedValue>,
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PsetNestedValue {
    Psets(Psets),
    Any(ValueWrap),
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Psets {
    Pset(Pset),
    PsetId(PsetId),
    Nested(HashMap<String, PsetNestedValue>),
}

#[pyclass]
struct PsetsIterator {
    entity: Psets,
    index: usize,
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF PsetId
// ----------------------------------------------------------------

#[pymethods]
impl PsetId {
    #[new]
    #[pyo3(signature = (/, *, id_))]
    pub fn new(id_: i64) -> PyResult<Self> {
        let result = Self { id_ };
        return Ok(result);
    }

    #[staticmethod]
    fn __class_name__() -> String {
        "PsetId".to_string()
    }

    #[staticmethod]
    #[pyo3(signature = (value, /))]
    pub fn model_validate<'a>(value: &Bound<'a, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn into_pyobject<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let raw = self.to_json().map_err(err_to_py_string)?;
        return ValueWrap(raw).into_pyobject(py);
    }
}

impl ToString for PsetId {
    fn to_string(&self) -> String {
        match self.to_json() {
            Ok(value) => match serde_json::to_string(&value) {
                Ok(text) => {
                    let name = Self::__class_name__();
                    return format!("{}({})", name, text);
                }
                Err(err) => {
                    panic!("{:?}", err);
                }
            },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}

impl JsonConversion<Value> for PsetId {
    fn from_json(value: &Value) -> Result<Self, String> {
        let raw = Self::deserialize(value.clone()).map_err(|err| format!("{}", err))?;
        return Ok(Self { id_: raw.id_ });
    }

    fn to_json(&self) -> Result<Value, String> {
        let result = json!({"id": self.id_});
        return Ok(result);
    }
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF Pset
// ----------------------------------------------------------------

#[pymethods]
impl Pset {
    #[new]
    #[pyo3(signature = (/, *, id_, class_, value, value_type=None))]
    pub fn new(
        id_: i64,
        class_: String,
        value: &Bound<'_, PyAny>,
        value_type: Option<String>,
    ) -> PyResult<Self> {
        let value = ValueWrap::extract_bound(value)?;
        let result = Self {
            id_,
            class_,
            value,
            value_type,
        };
        return Ok(result);
    }

    #[staticmethod]
    fn __class_name__() -> String {
        "Pset".to_string()
    }

    #[staticmethod]
    #[pyo3(signature = (value, /))]
    pub fn model_validate<'a>(value: &Bound<'a, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn into_pyobject<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let raw = self.to_json().map_err(err_to_py_string)?;
        return ValueWrap(raw).into_pyobject(py);
    }
}

impl ToString for Pset {
    fn to_string(&self) -> String {
        match self.to_json() {
            Ok(value) => match serde_json::to_string(&value) {
                Ok(text) => {
                    let name = Self::__class_name__();
                    return format!("{}({})", name, text);
                }
                Err(err) => {
                    panic!("{:?}", err);
                }
            },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}

impl JsonConversion<Value> for Pset {
    fn from_json(value: &Value) -> Result<Self, String> {
        let raw = Self::deserialize(value.clone()).map_err(err_to_string)?;
        return Ok(Self {
            id_: raw.id_,
            class_: raw.class_,
            value: raw.value,
            value_type: raw.value_type,
        });
    }

    fn to_json(&self) -> Result<Value, String> {
        let result = json!({
            "id": self.id_,
            "class": self.class_,
            "value": self.value,
            "value-type": self.value_type,
        });
        return Ok(result);
    }
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF PsetNestedValue
// ----------------------------------------------------------------

#[pymethods]
impl PsetNestedValue {
    #[new]
    #[pyo3(signature = (value, /))]
    pub fn new(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    #[staticmethod]
    #[pyo3(signature = (value, /))]
    pub fn model_validate(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl ToString for PsetNestedValue {
    fn to_string(&self) -> String {
        match self {
            Self::Psets(x) => {
                return x.to_string();
            }
            Self::Any(ValueWrap(x)) => {
                return x.to_string();
            }
        }
    }
}

impl JsonConversion<Value> for PsetNestedValue {
    fn from_json(value: &Value) -> Result<Self, String> {
        if let Ok(x) = Psets::from_json(value) {
            return Ok(Self::Psets(x));
        } else if let Ok(x) = ValueWrap::deserialize(value) {
            return Ok(Self::Any(x));
        } else {
            return Err("parse error".to_string());
        }
    }

    fn to_json(&self) -> Result<Value, String> {
        match self {
            Self::Psets(x) => {
                return x.to_json();
            }
            Self::Any(x) => {
                return x.to_json();
            }
        }
    }
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF PsetFlattenedValue + PsetFlattenedValueWithAddress
// ----------------------------------------------------------------

impl ToString for PsetFlattenedValue {
    fn to_string(&self) -> String {
        match self {
            PsetFlattenedValue::Pset(x) => x.to_string(),
            PsetFlattenedValue::PsetId(x) => x.to_string(),
            PsetFlattenedValue::Value(ValueWrap(x)) => x.to_string(),
        }
    }
}

impl ToString for PsetFlattenedValueWithAddress {
    fn to_string(&self) -> String {
        match (&self.addr, &self.entity) {
            (None, None) => {
                format!("Psets")
            }
            (Some(addr), None) => {
                format!("{}", addr)
            }
            (None, Some(entity)) => {
                format!("Psets: {}", entity.to_string())
            }
            (Some(addr), Some(entity)) => {
                format!("{}: {}", addr, entity.to_string())
            }
        }
    }
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS OF Psets
// ----------------------------------------------------------------

/// Rust implementation of struct
impl Psets {
    fn rust_flatten(
        &self,
        delimiter: Option<&String>,
        addr: Option<&String>,
    ) -> HashMap<String, PsetFlattenedValue> {
        let mut result = HashMap::<String, PsetFlattenedValue>::new();
        let delimiter = delimiter.map_or_else(|| ":".to_string(), |x| x.clone());
        let prefix = addr.map_or_else(|| "".to_string(), |x| format!("{}{}", x, delimiter));
        let addr = addr.map_or_else(|| "".to_string(), |x| x.clone());

        // iterate through cases
        match self {
            Psets::Pset(x) => {
                result.insert(addr, PsetFlattenedValue::Pset(x.clone()));
            }
            Psets::PsetId(x) => {
                result.insert(addr, PsetFlattenedValue::PsetId(x.clone()));
            }
            Psets::Nested(object) => {
                for (key, x) in object.iter() {
                    let subaddr = format!("{}{}", prefix, key);
                    match x {
                        PsetNestedValue::Psets(psets) => {
                            let subresult = psets.rust_flatten(Some(&delimiter), Some(&subaddr));
                            result.extend(subresult);
                        }
                        PsetNestedValue::Any(value) => {
                            result.insert(subaddr, PsetFlattenedValue::Value(value.clone()));
                        }
                    }
                }
            }
        }
        return result;
    }

    fn as_tree(&self, addr: Option<String>) -> GenericTree<PsetFlattenedValueWithAddress> {
        match self {
            Self::Pset(x) => {
                let node = PsetFlattenedValueWithAddress {
                    addr,
                    entity: Some(PsetFlattenedValue::Pset(x.clone())),
                };
                return GenericTree::new(node, None);
            }
            Self::PsetId(x) => {
                let node = PsetFlattenedValueWithAddress {
                    addr,
                    entity: Some(PsetFlattenedValue::PsetId(x.clone())),
                };
                return GenericTree::new(node, None);
            }
            Self::Nested(elements) => {
                let node = PsetFlattenedValueWithAddress { addr, entity: None };
                let mut t = GenericTree::new(node, None);
                for (subaddr, x) in elements.iter() {
                    match x {
                        PsetNestedValue::Psets(psets) => {
                            let child = psets.as_tree(Some(subaddr.clone()));
                            t.children.push(child);
                        }
                        PsetNestedValue::Any(ValueWrap(value)) => {
                            let node = PsetFlattenedValueWithAddress {
                                addr: Some(subaddr.clone()),
                                entity: Some(PsetFlattenedValue::Value(ValueWrap(value.clone()))),
                            };
                            let child = GenericTree::new(node, None);
                            t.children.push(child);
                        }
                    }
                }
                return t;
            }
        }
    }
}

/// Only for python implementation
#[pymethods]
impl Psets {
    #[new]
    #[pyo3(signature = (value, /))]
    pub fn new(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    fn __iter__(&self) -> PsetsIterator {
        return PsetsIterator::new(self).unwrap();
    }

    #[staticmethod]
    fn __class_name__() -> String {
        "Psets".to_string()
    }

    #[staticmethod]
    #[pyo3(signature = (value, /))]
    pub fn model_validate(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        let raw = ValueWrap::extract_bound(value)?;
        let value = raw.to_json().map_err(err_to_py_string)?;
        let result = Self::from_json(&value).map_err(err_to_py_string)?;
        return Ok(result);
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    #[pyo3(signature = (/, *, delimiter=":".to_string()))]
    fn flatten<'a>(
        &self,
        py: Python<'a>,
        delimiter: String,
    ) -> PyResult<HashMap<String, Bound<'a, PyAny>>> {
        let elements = self
            .rust_flatten(Some(&delimiter), None)
            .iter()
            .map(|(key, x)| match x {
                PsetFlattenedValue::Pset(x) => {
                    let value = x.into_pyobject(py).unwrap();
                    return (key.clone(), value);
                }
                PsetFlattenedValue::PsetId(x) => {
                    let value = x.into_pyobject(py).unwrap();
                    return (key.clone(), value);
                }
                PsetFlattenedValue::Value(value) => {
                    let value = value.clone();
                    let value = value.into_pyobject(py).unwrap();
                    return (key.clone(), value);
                }
            })
            .collect();
        return Ok(elements);
    }

    // #[pyo3(signature = ())]
    // fn walk<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
    //     match self {
    //         Psets::Pset(x) => {
    //             let key = ValueWrap(Value::Null).into_pyobject(py)?;
    //             let value = x.into_pyobject(py)?;
    //             let pair = PyTuple::new(py, &vec![key, value]);
    //             return PyList::new(py, &vec![pair]);
    //         }
    //         Psets::PsetId(x) => {
    //             let key = ValueWrap(Value::Null).into_pyobject(py)?;
    //             let value = x.into_pyobject(py)?;
    //             let pair = PyTuple::new(py, &vec![key, value]);
    //             return pair;
    //         }
    //         Psets::Nested(elements) => {
    //             let key = ValueWrap(Value::Null).into_pyobject(py)?;
    //             let value = x.into_pyobject(py)?;
    //             let pair = PyTuple::new(py, &vec![key, value]);
    //             return pair;
    //         }
    //     }
    // }
}

impl ToString for Psets {
    fn to_string(&self) -> String {
        let t = self.as_tree(None);
        return t.to_string();
    }
}

impl JsonConversion<Value> for Psets {
    fn from_json(value: &Value) -> Result<Self, String> {
        if let Ok(x) = Pset::from_json(value) {
            return Ok(Self::Pset(x));
        } else if let Ok(x) = PsetId::from_json(value) {
            return Ok(Self::PsetId(x));
        } else if let Value::Object(object) = value {
            let elements: HashMap<String, PsetNestedValue> = object
                .iter()
                .map(|(key, x)| {
                    if let Ok(value) = PsetNestedValue::from_json(x) {
                        return (key.clone(), value);
                    }
                    let value = ValueWrap::from_json(x).unwrap();
                    let value = PsetNestedValue::Any(value);
                    return (key.clone(), value);
                })
                .collect();
            return Ok(Self::Nested(elements));
        } else {
            return Err("invalid type".to_string());
        }
    }

    fn to_json(&self) -> Result<Value, String> {
        match self {
            Self::Pset(x) => {
                return x.to_json();
            }
            Self::PsetId(x) => {
                return x.to_json();
            }
            Self::Nested(elements) => {
                let items: Map<String, Value> = elements
                    .iter()
                    .map(|(key, x)| return (key.clone(), x.to_json().unwrap()))
                    .collect();
                let result = json!(items);
                return Ok(result);
            }
        }
    }
}

#[pymethods]
impl PsetsIterator {
    #[new]
    #[pyo3(signature = (entity, /))]
    fn new(entity: &Psets) -> PyResult<Self> {
        let entity = entity.clone();
        let result = Self { index: 0, entity };
        return Ok(result);
    }

    fn __iter__<'py>(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__<'py>(mut slf: PyRefMut<'py, Self>, py: Python<'py>) -> Option<Bound<'py, PyTuple>> {
        let result;
        let index = slf.index;
        match &slf.entity {
            Psets::Pset(x) => match index {
                0 => {
                    let key = ValueWrap(Value::Null).into_pyobject(py).unwrap().into_any();
                    let value = x.clone().into_pyobject(py).unwrap().into_any();
                    let pair = PyTuple::new(py, vec![key, value]).unwrap();
                    result = Some(pair);
                }
                _ => {
                    result = None;
                }
            },
            Psets::PsetId(x) => match index {
                0 => {
                    let key = ValueWrap(Value::Null).into_pyobject(py).unwrap().into_any();
                    let value = x.clone().into_pyobject(py).unwrap().into_any();
                    let pair = PyTuple::new(py, vec![key, value]).unwrap();
                    result = Some(pair);
                }
                _ => {
                    result = None;
                }
            },
            Psets::Nested(elements) => {
                result = elements.iter().nth(index).map(|(key, x)| {
                    let key = ValueWrap(Value::String(key.clone()))
                        .into_pyobject(py)
                        .unwrap()
                        .into_any();
                    let value = x.clone().into_pyobject(py).unwrap().into_any();
                    let pair = PyTuple::new(py, vec![key, value]).unwrap();
                    return pair;
                });
            }
        }
        slf.index += 1;

        return result;
    }
}
