#[macro_use]
extern crate lazy_static;

use dynomite::{dynamodb::*, *};
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request, Response,
};
use rusoto_core::Region;
use serde_json;
use std::env;

use serverless_products::Product;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

lazy_static! {
    static ref TABLE_NAME: String = env::var("TABLE_NAME").unwrap();
    static ref DYNAMODB: DynamoDbClient = DynamoDbClient::new(Region::default());
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    lambda::run(handler(get_products)).await?;

    Ok(())
}

async fn get_products(_event: Request, _ctx: Context) -> Result<impl IntoResponse, Err> {
    let products: Vec<Product> = DYNAMODB
        .scan(ScanInput {
            table_name: TABLE_NAME.to_string(),
            ..ScanInput::default()
        })
        .await
        .map(|output| {
            output
                .items
                .unwrap_or_default()
                .into_iter()
                .flat_map(|item| Product::from_attrs(item.clone()))
                .collect()
        })?;

    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&products).expect("failed to convert response body into JSON"))
        .expect("failed to render response"))
}
