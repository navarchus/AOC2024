use crate::structs::Rule;

pub fn question1(rules: &Vec<Rule>, updates: &Vec<Vec<u64>>) -> u64 {
    let valid = updates
        .iter()
        .filter(|update| rules.iter().all(|rule| rule.is_correctly_ordered(update)))
        .collect::<Vec<&Vec<u64>>>();

    let mut middle = 0;

    for update in valid {
        middle += update[update.len() / 2];
    }

    middle
}
