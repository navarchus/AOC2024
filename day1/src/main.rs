use std::{env, fs};

fn main() {
    //load dataset
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let in_file = fs::read_to_string(input_path).unwrap();
    let data: Vec<&str> = in_file.split_whitespace().collect();

    //get left and right sides of list, convert to int
    let mut lhs: Vec<u64> = data
        .iter()
        .step_by(2)
        .map(|val: &&str| val.parse::<u64>().unwrap())
        .collect();

    let mut rhs: Vec<u64> = data
        .iter()
        .skip(1)
        .step_by(2)
        .map(|val: &&str| val.parse::<u64>().unwrap())
        .collect();

    lhs.sort();
    rhs.sort();

    assert!(lhs.len() == rhs.len());

    //get total distance (not using reduce because brain hurt)
    let mut distance: u64 = 0;

    for el in lhs.iter().zip(rhs.iter()) {
        distance += (*el.0).abs_diff(*el.1)
    }

    println!("distance: {}", distance);

    //get total similarity, same excuse as above
    let mut sim: u64 = 0;

    for el in lhs.iter() {
        let right_count = rhs.iter().filter(|val| *el == **val).count() as u64;
        sim += *el * right_count;
    }
    println!("sim: {}", sim);
}
