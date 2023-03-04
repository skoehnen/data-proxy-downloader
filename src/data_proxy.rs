extern crate fstrings;
use fstrings::format_args_f;

use reqwest::blocking::Client;
use reqwest::Error;

pub struct Bucket {
    pub token: String,
    pub id: String
}

pub fn test() {
    println!("test called");
}

pub fn get_stat(bucket: Bucket) {
    println!("get_stat called");

    let stats_url = fstrings::f!("https://data-proxy.ebrains.eu/api/v1/buckets/{bucket.id}/stat");

    let client = Client::new();

    println!("{}", bucket.token);

    let request = client
        .get(stats_url)
        .header(reqwest::header::AUTHORIZATION, String::from(bucket.token));

    println!("{:?}", request);
    dbg!(request);

    //let response = request.send();

    //println!("{:?}", response);
}