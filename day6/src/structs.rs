use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
pub struct Direction(i32, i32);
impl Direction {
    pub const UP: Self = Self(0, -1);
    pub const DOWN: Self = Self(0, 1);
    pub const LEFT: Self = Self(-1, -0);
    pub const RIGHT: Self = Self(1, 0);
}

pub struct Guard {
    pub current_pos: (i32, i32),
    pub current_dir: Direction,
    pub has_seen: HashSet<(i32, i32)>,
}

impl Guard {
    pub fn new(start_x: i32, start_y: i32, start_dir: Direction) -> Guard {
        Guard {
            current_pos: (start_x, start_y),
            current_dir: start_dir,
            has_seen: HashSet::new(),
        }
    }
    pub fn move_forward(&mut self) {
        self.current_pos.0 += self.current_dir.0;
        self.current_pos.1 += self.current_dir.1;
    }
    pub fn move_backward(&mut self) {
        self.current_pos.0 -= self.current_dir.0;
        self.current_pos.1 -= self.current_dir.1;
    }
    pub fn rotate(&mut self) {
        match self.current_dir {
            Direction::UP => self.current_dir = Direction::RIGHT,
            Direction::RIGHT => self.current_dir = Direction::DOWN,
            Direction::DOWN => self.current_dir = Direction::LEFT,
            Direction::LEFT => self.current_dir = Direction::UP,
            _ => panic!("bad direction"),
        };
    }
    pub fn run_map(&mut self, map: Vec<Vec<char>>) -> &mut Guard {
        loop {
            self.move_forward();

            //check bounds
            if !self.is_in_bounds(
                (map[0].len(), map.len()),
                self.current_pos.0,
                self.current_pos.1,
            ) {
                break;
            }

            //current pos char
            let current_char = map[self.current_pos.1 as usize][self.current_pos.0 as usize];
            if current_char == '#' {
                self.move_backward();
                self.rotate();
            } else {
                self.has_seen.insert(self.current_pos);
            }
        }
        self
    }

    pub fn is_in_bounds(&self, size: (usize, usize), x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x as usize >= size.0 || y as usize >= size.1 {
            return false;
        }
        true
    }
}
