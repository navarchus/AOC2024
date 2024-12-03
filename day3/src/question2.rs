use regex::Regex;

pub fn question2(in_file: &String) -> usize {
    let mut dataset = in_file.clone();
    let mut result = 0;

    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let match_re = Regex::new(r"mul\((?<val_one>\d+),(?<val_two>\d+)\)").unwrap();

    let mut enabled = true;

    while dataset.chars().count() > 0 {
        if enabled {
            let dont_loc = dont_re.find(&dataset);
            let searchable_string: String = match dont_loc {
                Some(loc) => {
                    let res = dataset[0..loc.start()].to_string();
                    dataset = dataset[loc.end() + 1..].to_string();
                    res
                }
                None => {
                    let res = dataset.clone();
                    dataset = "".to_owned();
                    res
                }
            };
            result += match_re
                .captures_iter(&searchable_string)
                .map(|caps| {
                    let (__, [val_one, val_two]) = caps.extract();
                    (val_one, val_two)
                })
                .fold(0, |acc, val| {
                    acc + (val.0.parse::<usize>().unwrap() * val.1.parse::<usize>().unwrap())
                });
            enabled = false;
        } else {
            let do_loc = do_re.find(&dataset);

            match do_loc {
                Some(loc) => {
                    dataset = dataset[loc.end() + 1..].to_string();
                }
                None => {
                    dataset = "".to_owned();
                }
            }

            enabled = true;
        }
    }

    result
}
