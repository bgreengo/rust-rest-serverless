#[macro_use]
extern crate lazy_static;

use dynomite::{dynamodb::*, *};
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};
use rusoto_core::Region;
use serde_json;
use std::env;
use uuid::Uuid;

use serverless_products::Product;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

lazy_static! {
    static ref TABLE_NAME: String = env::var("TABLE_NAME").unwrap();
    static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::default());
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    lambda::run(handler(get_product)).await?;

    Ok(())
}

async fn get_product(event: Request, _ctx: Context) -> Result<impl IntoResponse, Err> {
    let path_parameters = event.path_parameters();
    let id = path_parameters
        .get("product_id")
        .expect("could not get product id");
    let id = Uuid::parse_str(&id)?;

    let product = Product {
        id: Some(id),
        ..Product::default()
    };

    let result = DYNAMODB
        .get_item(GetItemInput {
            table_name: TABLE_NAME.to_string(),
            key: product.key(),
            ..GetItemInput::default()
        })
        .await?;

    Ok(match result.item {
        Some(item) => {
            let product = Product::from_attrs(item)?;
            Response::builder()
                .status(200)
                .body(
                    serde_json::to_string(&product).expect("failed to convert response body into JSON"),
                )
                .expect("failed to render response")
        }
        None => Response::builder()
            .status(404)
            .body("File not found".to_string())
            .expect("failed to render response"),
    })
}
