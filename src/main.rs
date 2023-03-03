use std::io;
use std::env;
use std::cell::RefCell;

mod data_proxy;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut token = String::new();
    println!("Please enter your token:");
    io::stdin().read_line(&mut token).unwrap();
    
    let bucket = data_proxy::Bucket{
        token: token,
        id: args[1]
    };

    data_proxy::test();
    data_proxy::get_stat();
}
