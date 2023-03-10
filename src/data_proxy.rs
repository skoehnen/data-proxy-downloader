extern crate fstrings;
use fstrings::format_args_f;

use reqwest::blocking::Client;
//use reqwest::Error;
//use reqwest::header::HeaderValue;

use serde::{Deserialize};

#[derive(Clone)]
pub struct Bucket {
    pub token: String,
    pub id: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Status {
    pub name: String,
    pub objects_count: u64,
    pub bytes: u64,
    pub last_modified: String,
    pub is_public: bool,
    pub role: String,
    pub is_initialized: bool,
}

#[derive(Deserialize, Debug)]
pub struct Object {
    pub hash: String,
    last_modified: String,
    bytes: u64,
    name: String,
    content_type: String
}

#[derive(Deserialize, Debug)]
pub struct ObjectList {
    objects: Vec<Object>
}

pub fn test() {
    println!("test called");
}

pub fn get_stat(bucket: Bucket) -> Status {
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

    return status;
}

pub fn get_object_list(bucket: Bucket, status: Status) -> ObjectList{
    let object_list_url = fstrings::f!("https://data-proxy.ebrains.eu/api/v1/buckets/{bucket.id}?limit={status.objects_count}");

    let client = Client::new();

    let response = client
        .get(object_list_url)
        .bearer_auth(bucket.token)
        .send()
        .unwrap();

    let object_list: ObjectList = response.json::<ObjectList>().unwrap();

    return object_list;
}