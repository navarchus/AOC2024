use std::collections::HashMap;

fn parse_nums(input: &String) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.to_string().parse().unwrap())
        .collect()
}

fn blink (stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (k,v) in stones {
        match k {
            0 => *new_stones.entry(1).or_default() += v,
            _ => {
                let num_str = k.to_string();
                if num_str.len() % 2 == 0 {
                    let lhs: usize = num_str[0..num_str.len()/2].parse().unwrap();
                    let rhs: usize = num_str[(num_str.len()/2)..].parse().unwrap();
                    *new_stones.entry(lhs).or_default() += v;
                    *new_stones.entry(rhs).or_default() += v;
                } else {
                    *new_stones.entry(2024 * k).or_default() += v;
                }
            }
        }
    }

    new_stones
}

pub fn solve(input: &String) -> usize {
    let mut stones = HashMap::new();

    for n in parse_nums(input) {
        stones.entry(n).and_modify(|val| *val += 1).or_insert(1);
    }

    for _ in 0..75 {
        stones = blink(stones);
    }

    stones.values().sum()
}
