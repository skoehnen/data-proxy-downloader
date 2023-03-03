use std::io;
use std::env;

mod data_proxy;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let mut token = String::new();
    println!("Please enter your token:");
    io::stdin().read_line(&mut token).unwrap();
    
    data_proxy::test();
    data_proxy::get_stat();
}
