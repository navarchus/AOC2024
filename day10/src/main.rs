use std::{env, fs};

pub mod util;
mod question1;
mod question2;


fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();
    let lines: Vec<&str> = in_file.lines().collect();
    
    println!("question 1: {}", question1::solve(&lines));
    println!("question 2: {}", question2::solve(&lines));
}
