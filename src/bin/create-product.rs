#[macro_use]
extern crate lazy_static;

use dynomite::dynamodb::DynamoDbClient;
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};
use rusoto_core::Region;
use std::env;
use std::sync::Arc;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

lazy_static! {
    static ref TABLE_NAME: String = env::var("TABLE_NAME").unwrap();
    static ref DYNAMODB: Arc<DynamoDbClient> = Arc::new(DynamoDbClient::new(Region::default()));
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    lambda::run(handler(create_product)).await?;

    Ok(())
}

async fn create_product(event: Request, _ctx: Context) -> Result<impl IntoResponse, Err> {
    let product: products::Product = match event.payload()? {
        Some(product) => product,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body("Missing body".to_string())?)
        }
    };

    let product = products::create_product(DYNAMODB.clone(), &TABLE_NAME, product).await?;

    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&product)?)?)
}
