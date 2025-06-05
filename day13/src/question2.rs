use super::utils::{ClawMachine};


pub fn solve(input: &Vec<ClawMachine>) -> i64 {
    let mut token_total = 0;

    for machine in input {
        let mut new_mach = machine.clone();
        new_mach.prize.x += 10000000000000;
        new_mach.prize.y += 10000000000000;

        token_total +=  new_mach.solve_machine_tokens();
    }

    token_total
}