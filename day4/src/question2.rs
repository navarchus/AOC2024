use std::collections::HashMap;

pub fn in_bounds<T>(val: T, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    val >= min && val < max
}

pub fn question2(char_vec: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let valid_chars = HashMap::from([('M', 'S'), ('S', 'M')]);
    for y in 0..char_vec.len() {
        for x in 0..char_vec[0].len() {
            if !in_bounds(y, 1, char_vec.len() - 1) || !in_bounds(x, 1, char_vec[0].len() - 1) {
                continue;
            };

            let corners = [
                (char_vec[y - 1][x - 1], char_vec[y + 1][x + 1]),
                (char_vec[y - 1][x + 1], char_vec[y + 1][x - 1]),
            ];
            if char_vec[y][x] == 'A' {
                let found = corners.iter().all(|corner| {
                    valid_chars.contains_key(&corner.0)
                        && *valid_chars.get(&corner.0).unwrap() == corner.1
                });
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}
