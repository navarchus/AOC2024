// mod question1;
// mod question2;
mod util;
mod question1;
mod question2;

use std::{env, fs};
use question1::question1;
use question2::question2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();

    println!("question 1: {:?}", question1(&in_file));
    println!("question 2: {:?}", question2(&in_file));
}
