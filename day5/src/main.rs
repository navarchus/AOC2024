mod question1;
mod question2;
mod structs;

use question1::question1;
use question2::question2;
use regex::Regex;
use std::{env, fs};
use structs::Rule;

fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();
    let lines: Vec<&str> = in_file.lines().collect();

    let rules_re = Regex::new(r"(?-m)^(\d)+\|(\d)+$").unwrap();
    let rules: Vec<Rule> = lines
        .iter()
        .cloned()
        .filter(|line| rules_re.is_match(line))
        .map(|rule_string| {
            let split_rule_string: Vec<&str> = rule_string.split("|").collect();
            Rule {
                first: split_rule_string[0].parse::<u64>().unwrap(),
                second: split_rule_string[1].parse::<u64>().unwrap(),
            }
        })
        .collect();

    let updates_re = Regex::new(r"^(\d+,)*(\d)+$").unwrap();
    let updates: Vec<Vec<u64>> = lines
        .iter()
        .cloned()
        .filter(|line| updates_re.is_match(&line))
        .map(|update_string| {
            update_string
                .split(",")
                .map(|el| el.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    println!("question 1: {:?}", question1(&rules, &updates));
    println!("question 2: {:?}", question2(&rules, &updates));
}
