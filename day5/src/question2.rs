use std::{collections::HashMap, u64};

use crate::structs::Rule;
pub fn question2(rules: &Vec<Rule>, updates: &Vec<Vec<u64>>) -> u64 {
    let mut rule_hashmap: HashMap<u64, Vec<u64>> = HashMap::new();
    for rule in rules {
        rule_hashmap
            .entry(rule.first)
            .or_default()
            .push(rule.second);
    }

    let made_valid = updates
        .iter()
        .filter(|update| {
            rules
                .iter()
                .any(|rule| !(rule.is_correctly_ordered(update)))
        })
        .map(|invalid_update| {
            let mut valid_update = invalid_update.clone();
            valid_update.sort_by(|a, b| {
                if rule_hashmap.contains_key(a) {
                    if rule_hashmap.get(a).unwrap().contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                }
                return std::cmp::Ordering::Equal;
            });
            valid_update
        })
        .collect::<Vec<Vec<u64>>>();

    let mut middle = 0;

    for update in made_valid {
        middle += update[update.len() / 2];
    }

    middle
}
