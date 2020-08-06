use dynomite::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Item, Clone)]
pub struct Product {
    #[dynomite(partition_key)]
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            id: Some(Uuid::new_v4()),
            name: "".to_string(),
            description: "".to_string(),
        }
    }
}
