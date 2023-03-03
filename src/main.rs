use std::io;
use std::env;

mod data_proxy;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Hello, world!");
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    println!("User input: {}", user_input);
    data_proxy::test();
    data_proxy::get_stat();
}
