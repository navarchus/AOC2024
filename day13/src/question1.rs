use super::utils::{ClawMachine};


pub fn solve(input: &Vec<ClawMachine>) -> i64 {
    let mut token_total = 0;

    for machine in input {
        token_total += machine.solve_machine_tokens();
    }

    token_total
}