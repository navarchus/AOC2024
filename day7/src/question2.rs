fn can_eval(desired: u64, components: &Vec<u64>, index: usize, acc: u64) -> bool {
    if index == components.len() {
        return acc == desired;
    }

    if acc > desired {
        return false;
    }

    //add
    if can_eval(desired, components, index + 1, acc + components[index]) {
        return true;
    }

    //mul
    if can_eval(desired, components, index + 1, acc * components[index]) {
        return true;
    }

    //concat
    if can_eval(
        desired,
        components,
        index + 1,
        (acc.to_string() + &components[index].to_string())
            .parse()
            .unwrap(),
    ) {
        return true;
    }

    false
}

pub fn question2(input: &Vec<&str>) -> u64 {
    let mut count = 0;

    for line in input {
        //parse input
        let line_vec: Vec<&str> = line.split(':').collect();
        let desired: u64 = line_vec[0].parse().unwrap();
        let components: Vec<u64> = line_vec[line_vec.len() - 1]
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        if can_eval(desired, &components, 1, components[0]) {
            count += desired;
        }
    }

    count
}
