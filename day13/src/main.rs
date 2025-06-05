pub mod utils;
mod question1;
mod question2;

use std::{env, fs};



fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_string = fs::read_to_string(input_path).unwrap();

    let machines = utils::parse_claw_machines(&in_string);

    println!("question 1: {}" , question1::solve(&machines));
    println!("question 2: {}", question2::solve(&machines));
}
