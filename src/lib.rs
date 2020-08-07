use dynomite::{
    dynamodb::{DeleteItemInput, DynamoDb, GetItemInput, PutItemInput, ScanInput},
    *,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Serialize, Deserialize, Item, Clone, PartialEq)]
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

/// Returns a vector of products from the table
pub async fn get_products(db: Arc<impl DynamoDb>, table_name: &str) -> Result<Vec<Product>, Err> {
    let products: Vec<Product> = db
        .scan(ScanInput {
            table_name: table_name.to_string(),
            ..ScanInput::default()
        })
        .await
        .map(|output| {
            output
                .items
                .unwrap_or_default()
                .into_iter()
                .flat_map(Product::from_attrs)
                .collect()
        })?;

    Ok(products)
}

/// Returns a single product from the table
pub async fn get_product(
    db: Arc<impl DynamoDb>,
    table_name: &str,
    id: Uuid,
) -> Result<Option<Product>, Err> {
    let product = Product {
        id: Some(id),
        ..Product::default()
    };

    let result = db
        .get_item(GetItemInput {
            table_name: table_name.to_string(),
            key: product.key(),
            ..GetItemInput::default()
        })
        .await?;

    Ok(match result.item {
        Some(item) => Some(Product::from_attrs(item)?),
        None => None,
    })
}

// Create a new product
pub async fn create_product(
    db: Arc<impl DynamoDb>,
    table_name: &str,
    product: Product,
) -> Result<Product, Err> {
    let product = Product {
        id: Some(Uuid::new_v4()),
        ..product
    };

    db.put_item(PutItemInput {
        table_name: table_name.to_string(),
        item: product.clone().into(),
        ..PutItemInput::default()
    })
    .await?;

    Ok(product)
}

// Delete a product
pub async fn delete_product(db: Arc<impl DynamoDb>, table_name: &str, id: Uuid) -> Result<(), Err> {
    let product = Product {
        id: Some(id),
        ..Product::default()
    };

    db.delete_item(DeleteItemInput {
        table_name: table_name.to_string(),
        key: product.key(),
        ..DeleteItemInput::default()
    })
    .await?;

    Ok(())
}
