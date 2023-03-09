use std::io;
use std::env;

mod data_proxy;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let bucket_id = String::from(&args[1]);

    let mut token = String::new();
    println!("Please enter your token:");
    io::stdin().read_line(&mut token).unwrap();
    
    let bucket = data_proxy::Bucket{
        token: token.trim().to_string(),
        id: bucket_id
    };

    data_proxy::test();
    let status: data_proxy::Status = data_proxy::get_stat(bucket);
}
