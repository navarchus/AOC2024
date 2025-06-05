fn add_tuples<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: std::ops::Add<Output = T>,
{
    (a.0 + b.0, a.1 + b.1)
}
//789
//456
//123

fn check_neighbors(chars: &Vec<Vec<char>>, idx_x: usize, idx_y: usize) -> i64 {
    let mut sum = 0;
    let _curr_char = chars[idx_y][idx_x];

    let char_neighbors = [
        //up left:
        if idx_x == 0 || idx_y == 0 {
            None
        } else {
            Some(chars[idx_y - 1][idx_x - 1])
        },
        //directly above
        if idx_y == 0 {
            None
        } else {
            Some(chars[idx_y - 1][idx_x])
        },
        //up right:
        if idx_y == 0 || idx_x + 1 >= chars[idx_y].len() {
            None
        } else {
            Some(chars[idx_y - 1][idx_x + 1])
        },
        //directly left
        if idx_x == 0 {
            None
        } else {
            Some(chars[idx_y][idx_x - 1])
        },
        //middle
        Some(chars[idx_y][idx_x]),
        //directly right
        if idx_x + 1 >= chars[idx_y].len() {
            None
        } else {
            Some(chars[idx_y][idx_x + 1])
        },
        //down left:
        if idx_x == 0 || idx_y + 1 >= chars.len() {
            None
        } else {
            Some(chars[idx_y + 1][idx_x - 1])
        },
        //directly below:
        if idx_y + 1 >= chars.len() {
            None
        } else {
            Some(chars[idx_y + 1][idx_x])
        },
        //down right:
        if idx_y + 1 >= chars.len() || idx_x + 1 >= chars[idx_y].len() {
            None
        } else {
            Some(chars[idx_y + 1][idx_x + 1])
        },
    ];
    //012
    //345
    //678

    //is there a corner? if so, add a side.

    //up right outer corner (145 != 2)
    if (char_neighbors[1] == char_neighbors[4])
        && (char_neighbors[4] == char_neighbors[5])
        && (char_neighbors[4] != char_neighbors[2])
    {
        sum += 1
    }

    //down right outer corner (547 != 8)
    if (char_neighbors[5] == char_neighbors[4])
        && (char_neighbors[4] == char_neighbors[7])
        && (char_neighbors[4] != char_neighbors[8])
    {
        sum += 1
    }

    //down left outer corner (143 != 0)
    if (char_neighbors[1] == char_neighbors[4])
        && (char_neighbors[4] == char_neighbors[3])
        && (char_neighbors[4] != char_neighbors[0])
    {
        sum += 1
    }

    //down up outer corner (347 != 6)
    if (char_neighbors[3] == char_neighbors[4])
        && (char_neighbors[4] == char_neighbors[7])
        && (char_neighbors[4] != char_neighbors[6])
    {
        sum += 1
    }
    
    // INNER CORNERS

    //up right inner corner (1&5!=4)
    if (char_neighbors[1] != char_neighbors[4]) && (char_neighbors[5] != char_neighbors[4]) {
        sum += 1;
    }

    //bottom right inside corner (5&7!=4)
    if (char_neighbors[5] != char_neighbors[4]) && (char_neighbors[7] != char_neighbors[4]) {
        sum += 1;
    }

    //bottom left inside corner (3&7 != 4)
    if (char_neighbors[3] != char_neighbors[4]) && (char_neighbors[7] != char_neighbors[4]) {
        sum += 1;
    }

    //up left inside corner (1&3 != 4)
    if (char_neighbors[1] != char_neighbors[4]) && (char_neighbors[3] != char_neighbors[4]) {
        sum += 1;
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
                let (area, sides) = get_region_params(&chars, &mut seen, idx_x, idx_y, 0, 0);

                // println!();
                // println!("{}", chars[idx_y][idx_x]);
                // println!("area: {area}, sides: {sides}");

                res += area * sides;
            }
        }
    }
    res
}
