pub fn in_bounds<T>(val: T, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    val >= min && val < max
}

pub fn question1(char_vec: &Vec<Vec<char>>) -> u64 {
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut count = 0;
    let desired = "XMAS";

    for y in 0..char_vec.len() {
        for x in 0..char_vec[0].len() {
            if char_vec[y][x] == 'X' {
                for (dir_x, dir_y) in dirs {
                    let found = desired.chars().enumerate().all(|(i, search_char)| {
                        let search_x = x as i32 + (i as i32) * dir_x;
                        let search_y = y as i32 + (i as i32) * dir_y;

                        in_bounds(search_x, 0, char_vec[0].len() as i32)
                            && in_bounds(search_y, 0, char_vec.len() as i32)
                            && search_char == char_vec[search_y as usize][search_x as usize]
                    });
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
