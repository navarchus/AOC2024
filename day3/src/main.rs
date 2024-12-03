mod question1;
mod question2;

use question1::question1;
use question2::question2;
use std::{env, fs};

fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();

    println!("Total match no do/dont: {}", question1(&in_file));
    println!("Total match with do/dont: {}", question2(&in_file));
}
