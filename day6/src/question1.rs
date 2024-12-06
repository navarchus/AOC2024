use crate::structs::{Direction, Guard};

pub fn question1(char_vec: &Vec<Vec<char>>) -> u64 {
    let curr_vec = char_vec.clone();
    let dir_list = ['^', 'v', '>', '<'];
    //find start pos
    let mut start_pos_x = 0;
    let mut start_pos_y = 0;
    let mut start_direction = Direction::UP;

    for y in 0..curr_vec.len() {
        for x in 0..curr_vec[0].len() {
            for dir in dir_list {
                if curr_vec[y][x] == dir {
                    start_pos_x = x;
                    start_pos_y = y;
                    match dir {
                        '^' => start_direction = Direction::UP,
                        'v' => start_direction = Direction::DOWN,
                        '<' => start_direction = Direction::LEFT,
                        '>' => start_direction = Direction::RIGHT,
                        _ => panic!("bad dir!"),
                    }
                    break;
                }
            }
        }
    }

    let mut guard = Guard::new(start_pos_x as i32, start_pos_y as i32, start_direction);
    guard.run_map(curr_vec);

    guard.has_seen.len() as u64
}
