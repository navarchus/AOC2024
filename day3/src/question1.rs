use regex::Regex;

pub fn question1(in_file: &String) -> u64 {
    let re = Regex::new(r"mul\((?<val_one>\d+),(?<val_two>\d+)\)").unwrap();

    let results = re
        .captures_iter(&in_file)
        .map(|caps| {
            let (__, [val_one, val_two]) = caps.extract();
            (val_one, val_two)
        })
        .fold(0, |acc, val| {
            acc + (val.0.parse::<u64>().unwrap() * val.1.parse::<u64>().unwrap())
        });

    results
}
