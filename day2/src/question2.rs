fn check_increasing(v: &Vec<u32>) -> bool {
    for item in v.iter().zip(v.iter().skip(1)) {
        if *item.0 > *item.1 {
            return false;
        }
    }
    return true;
}

fn check_decreasing(v: &Vec<u32>) -> bool {
    for item in v.iter().zip(v.iter().skip(1)) {
        if *item.0 < *item.1 {
            return false;
        }
    }
    return true;
}

fn check_diff(v: &Vec<u32>) -> bool {
    let mut max_diff = 0;
    let mut min_diff = 0;
    for item in v.iter().zip(v.iter().skip(1)) {
        if item.0.abs_diff(*item.1) > max_diff {
            max_diff = item.0.abs_diff(*item.1);
        }
        if item.0.abs_diff(*item.1) < min_diff {
            min_diff = item.0.abs_diff(*item.1);
        }
        if item.0 == item.1 {
            return false;
        }
    }
    if (min_diff == 0 && max_diff == 0) || max_diff > 3 {
        return false;
    }
    return true;
}

fn check_report(report: &Vec<u32>) -> bool {
    let is_increasing: bool = check_increasing(report);
    let is_decreasing: bool = check_decreasing(report);
    let diff_correct: bool = check_diff(report);
    if (is_decreasing ^ is_increasing) && diff_correct {
        return true;
    }
    return false;
}

pub fn question2(parsed_data: &Vec<Vec<u32>>) {
    let mut safe_count = 0;

    for report in parsed_data {
        if check_report(report) {
            safe_count += 1;
        } else {
            for x in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(x);
                if check_report(&new_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe With PD: {}", safe_count)
}
