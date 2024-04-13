use serde::{Deserialize, Serialize};

/// AlgoBcrypt
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoBcrypt {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,
}
