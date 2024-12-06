use crate::structs::{Direction, Guard};

fn check_loop(char_vec: &Vec<Vec<char>>, mut guard: Guard) -> bool {
    let mut repeated_steps = 0;
    loop {
        guard.move_forward();

        //check bounds
        if !guard.is_in_bounds(
            (char_vec[0].len(), char_vec.len()),
            guard.current_pos.0,
            guard.current_pos.1,
        ) {
            break;
        }

        //current pos char
        let current_char: char =
            char_vec[guard.current_pos.1 as usize][guard.current_pos.0 as usize];

        //uptick if we have already visited this location
        if guard.has_seen.contains(&guard.current_pos) {
            repeated_steps += 1;
            // if the count has reached double the visited assume loop is looping
            if repeated_steps == guard.has_seen.len() * 2 {
                return true;
            }
        }

        if current_char == '#' {
            guard.move_backward();
            guard.rotate();
        } else {
            guard.has_seen.insert(guard.current_pos);
        }
    }
    false
}

pub fn question2(char_vec: &Vec<Vec<char>>) -> u64 {
    let dir_list = ['^', 'v', '>', '<'];
    //find start pos
    let mut start_pos_x = 0;
    let mut start_pos_y = 0;
    let mut start_direction = Direction::UP;

    for y in 0..char_vec.len() {
        for x in 0..char_vec[0].len() {
            for dir in dir_list {
                if char_vec[y][x] == dir {
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

    let mut sum = 0;
    for y in 0..char_vec.len() {
        for x in 0..char_vec[y].len() {
            let mut new_vec = char_vec.clone();
            if new_vec[y][x] == '#' {
                continue;
            }
            new_vec[y][x] = '#';
            if check_loop(
                &new_vec,
                Guard::new(start_pos_x as i32, start_pos_y as i32, start_direction),
            ) {
                sum += 1;
            }
        }
    }

    sum
}
