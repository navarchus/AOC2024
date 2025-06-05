fn add_tuples<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: std::ops::Add<Output = T>,
{
    (a.0 + b.0, a.1 + b.1)
}

fn check_neighbors(chars: &Vec<Vec<char>>, idx_x: usize, idx_y: usize) -> i64 {
    let mut sum = 4;

    //up
    if idx_y != 0 && chars[idx_y - 1][idx_x] == chars[idx_y][idx_x] {
        sum -= 1;
    }

    //right
    if idx_x + 1 < chars[idx_y].len() && chars[idx_y][idx_x + 1] == chars[idx_y][idx_x] {
        sum -= 1;
    }

    //down
    if idx_y + 1 < chars.len() && chars[idx_y + 1][idx_x] == chars[idx_y][idx_x] {
        sum -= 1;
    }

    //left
    if idx_x != 0 && chars[idx_y][idx_x - 1] == chars[idx_y][idx_x] {
        sum -= 1;
    }
    sum
}

fn get_region_params(
    chars: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    idx_x: usize,
    idx_y: usize,
    area: i64,
    perimeter: i64,
) -> (i64, i64) {
    //mark as seen
    seen[idx_y][idx_x] = true;
    let curr_char = chars[idx_y][idx_x];
    let mut res_tuple = (1, check_neighbors(chars, idx_x, idx_y));

    //up
    if idx_y != 0 && chars[idx_y - 1][idx_x] == curr_char && !seen[idx_y - 1][idx_x] {
        res_tuple = add_tuples(
            res_tuple,
            get_region_params(chars, seen, idx_x, idx_y - 1, area, perimeter),
        );
    }

    //right
    if idx_x + 1 < chars[idx_y].len()
        && chars[idx_y][idx_x + 1] == curr_char
        && !seen[idx_y][idx_x + 1]
    {
        res_tuple = add_tuples(
            res_tuple,
            get_region_params(chars, seen, idx_x + 1, idx_y, area, perimeter),
        );
    }

    //down
    if idx_y + 1 < chars.len() && chars[idx_y + 1][idx_x] == curr_char && !seen[idx_y + 1][idx_x] {
        res_tuple = add_tuples(
            res_tuple,
            get_region_params(chars, seen, idx_x, idx_y + 1, area, perimeter),
        );
    }
    //left
    if idx_x != 0 && chars[idx_y][idx_x - 1] == curr_char && !seen[idx_y][idx_x - 1] {
        res_tuple = add_tuples(
            res_tuple,
            get_region_params(chars, seen, idx_x - 1, idx_y, area, perimeter),
        );
    }

    res_tuple
}

pub fn solve(input: &String) -> i64 {
    let mut seen: Vec<Vec<bool>> = input.lines().map(|l| vec![false; l.len()]).collect();
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut res = 0;

    for idx_y in 0..chars.len() {
        for idx_x in 0..chars[idx_y].len() {
            if !seen[idx_y][idx_x] {
                // println!("{}", chars[idx_y][idx_x]);
                let (area, perim) = get_region_params(&chars, &mut seen, idx_x, idx_y, 0, 0);
                // println!("area: {area}, perim: {perim}");
                res += area * perim;
            }
        }
    }
    res
}
