#[macro_use]
extern crate lazy_static;

use dynomite::dynamodb::DynamoDbClient;
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request, RequestExt,
};
use rusoto_core::Region;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

lazy_static! {
    static ref TABLE_NAME: String = env::var("TABLE_NAME").unwrap();
    static ref DYNAMODB: Arc<DynamoDbClient> = Arc::new(DynamoDbClient::new(Region::default()));
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    lambda::run(handler(delete_product)).await?;

    Ok(())
}

async fn delete_product(event: Request, _ctx: Context) -> Result<impl IntoResponse, Err> {
    let path_parameters = event.path_parameters();
    let id = path_parameters
        .get("product_id")
        .expect("could not get product id");
    let id = Uuid::parse_str(&id)?;

    products::delete_product(DYNAMODB.clone(), &TABLE_NAME, id).await?;

    Ok("Product deleted")
}
