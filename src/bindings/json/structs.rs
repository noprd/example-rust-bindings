/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// ----------------------------------------------------------------
/// STRUCTURES/TYPES
/// ----------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueWrap(pub Value);
