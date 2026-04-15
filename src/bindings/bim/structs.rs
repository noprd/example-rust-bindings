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

use crate::bindings::err_to_string;
use crate::bindings::json::ValueWrap;
use crate::models::tree::GenericTree;

// ----------------------------------------------------------------
// STRUCTS
// ----------------------------------------------------------------

#[pyclass(from_py_object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PsetId {
    #[pyo3(get, set)]
    #[serde(alias = "id")]
    pub id_: i64,
}

#[pyclass(from_py_object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pset {
    #[pyo3(get, set)]
    #[serde(alias = "id")]
    pub id_: i64,
    #[pyo3(get, set)]
    #[serde(alias = "class")]
    pub class_: String,
    // need this to be able to handle pyo3 traits
    #[pyo3(get, set)]
    #[serde(alias = "value")]
    pub value: ValueWrap,
    #[pyo3(get, set)]
    #[serde(alias = "value-type")]
    pub value_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PsetFlattenedValue {
    Pset(Pset),
    PsetId(PsetId),
    Value(ValueWrap),
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PsetFlattenedValueWithAddress {
    pub addr: Option<String>,
    pub entity: Option<PsetFlattenedValue>,
}

#[pyclass(from_py_object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PsetNestedValue {
    Psets(Psets),
    Any(ValueWrap),
}

#[pyclass(from_py_object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Psets {
    Pset(Pset),
    PsetId(PsetId),
    Nested(HashMap<String, PsetNestedValue>),
}

#[pyclass]
pub struct PsetsIterator {
    pub entity: Psets,
    pub index: usize,
}
