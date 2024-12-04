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
    let char_vec: Vec<Vec<char>> = in_file.lines().map(|line| line.chars().collect()).collect();

    println!("question 1: {}", question1(&char_vec));
    println!("question 2: {}", question2(&char_vec));
}
