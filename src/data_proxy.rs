extern crate fstrings;
use fstrings::format_args_f;

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

    println!("{}", stats_url);
}