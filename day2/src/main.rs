mod question1;
mod question2;
use std::{env, fs, u32};

use question1::question1;
use question2::question2;
fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let binding = fs::read_to_string(input_path).unwrap();
    let data: Vec<&str> = binding.lines().collect();

    //get numerical representation
    let parsed_data: Vec<Vec<u32>> = data
        .iter()
        .map(|val| {
            val.split_whitespace()
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    question1(&parsed_data);
    question2(&parsed_data);
}
