//! Testing resources on AWS
//! ========================
//!
//! This assumes that there is an environment variable called `REST_API`
//! containing the link to the API url.

use products::Product;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::StatusCode;
use std::env;

fn get_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

#[test]
fn test_flow() {
    let client = reqwest::blocking::Client::new();
    let rest_api: String = env::var("REST_API").unwrap();

    // Initialize a product
    let req_product = Product {
        name: get_random_string(10),
        description: get_random_string(100),
        ..Product::default()
    };

    // Create a new product
    let res = client.post(&rest_api).json(&req_product).send().unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Compare the returned product, validate it has an ID
    let res_product: Product = res.json().unwrap();
    assert!(res_product.id.is_some());
    assert_eq!(req_product.name, res_product.name);
    assert_eq!(req_product.description, res_product.description);

    // Retrieve the product
    let res = client
        .get(&format!(
            "{}{}",
            rest_api,
            res_product.id.unwrap().to_string()
        ))
        .send()
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let get_product: Product = res.json().unwrap();
    assert_eq!(get_product, res_product);

    // Gathering all products should return at least 1 item
    let res = client.get(&rest_api).send().unwrap();
    let products: Vec<Product> = res.json().unwrap();
    assert!(products.len() > 0);

    // Delete the product
    let res = client
        .delete(&format!(
            "{}{}",
            rest_api,
            res_product.id.unwrap().to_string()
        ))
        .send()
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Try retrieving again
    let res = client
        .get(&format!(
            "{}{}",
            rest_api,
            res_product.id.unwrap().to_string()
        ))
        .send()
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
