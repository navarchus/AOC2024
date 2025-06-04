use std::{env, fs};

mod question1;
mod question2;
pub mod util;

fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();
    // let lines: Vec<&str> = in_file.lines().collect();

    println!("question 1: {}", question1::solve(&in_file));
    println!("question 2: {}", question2::solve(&in_file));
}
