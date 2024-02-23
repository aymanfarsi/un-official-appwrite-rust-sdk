use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Attributes List
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeList {
    /// Total number of attributes in the given collection.
    total: u64,
    /// List of attributes.
    attributes: Value,
}
