use serde::{Deserialize, Serialize};

use super::database::Database;

/// Databases List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DatabaseList {
    /// Total number of databases documents that matched your query.
   pub     total: u64,
    /// List of databases.
   pub     databases: Vec<Database>,
}
