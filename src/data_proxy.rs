extern crate fstrings;
use fstrings::format_args_f;

use reqwest::blocking::Client;
//use reqwest::Error;
//use reqwest::header::HeaderValue;

use serde::{Deserialize, Serialize};

pub struct Bucket {
    pub token: String,
    pub id: String
}

#[derive(Debug, Deserialize)]
struct Status {
    name: String,
    objects_count: u64,
    bytes: u64,
    last_modified: String,
    is_public: bool,
    role: String,
    is_initialized: bool,
}

pub fn test() {
    println!("test called");
}

pub fn get_stat(bucket: Bucket) {
    println!("get_stat called");

    let stats_url = fstrings::f!("https://data-proxy.ebrains.eu/api/v1/buckets/{bucket.id}/stat");

    let client = Client::new();

    println!("{}", bucket.token);

    //let val = HeaderValue::from_str("Hallo Tim");

    //let val = HeaderValue::from_str(&format!("Bearer {}", bucket.token.clone()));

    //dbg!(val);

    let response = client
        .get(stats_url)
        .bearer_auth(bucket.token)
        .send()
        .unwrap();

    //dbg!(response.text());
    let status: Status = response.json::<Status>().unwrap();

    dbg!(status);
}