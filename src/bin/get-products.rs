#[macro_use]
extern crate lazy_static;

use dynomite::dynamodb::DynamoDbClient;
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request, Response,
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
    lambda::run(handler(get_products)).await?;

    Ok(())
}

async fn get_products(_event: Request, _ctx: Context) -> Result<impl IntoResponse, Err> {
    let products = products::get_products(DYNAMODB.clone(), &TABLE_NAME).await?;

    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&products).expect("failed to convert response body into JSON"))
        .expect("failed to render response"))
}
