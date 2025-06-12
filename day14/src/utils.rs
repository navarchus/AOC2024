#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pub pos: Point,
    vel: Point,
}

#[derive(PartialEq, Debug)]
pub enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Robot {
    pub fn step(&self, bounds: (usize, usize)) -> Robot {
        let mut res = self.clone();

        //move x
        for _ in 0..res.vel.x.abs() {
            res.pos.x += 1 * res.vel.x.signum();
            if res.pos.x >= (bounds.0 as i64) {
                res.pos.x = 0;
            } else if res.pos.x < 0 {
                res.pos.x = (bounds.0 as i64) - 1;
            }
        }

        //move y
        for _ in 0..res.vel.y.abs() {
            res.pos.y += 1 * res.vel.y.signum();
            if res.pos.y >= (bounds.1 as i64) {
                res.pos.y = 0;
            } else if res.pos.y < 0 {
                res.pos.y = (bounds.1 as i64) - 1
            }
        }

        res
    }

    pub fn get_quadrant(&self, bounds: (usize, usize)) -> Option<Quadrant> {
        //Q1 | Q2
        // _____
        //Q3 | Q4

        let mid_x = if bounds.0 % 2 == 0 {
            panic!("invalid midpoint for x dir! odd numbers only")
        } else {
            ((bounds.0) as f64 / 2f64).floor() as i64
        };
        let mid_y = if bounds.0 % 2 == 0 {
            panic!("invalid midpoint for y dir! odd numbers only")
        } else {
            ((bounds.1) as f64 / 2f64).floor() as i64
        };

        //at midpoints
        if self.pos.x == mid_x || self.pos.y == mid_y {
            return None;
        }

        //Q1
        if self.pos.x < mid_x && self.pos.y < mid_y {
            return Some(Quadrant::Q1);
        }

        //Q2
        if self.pos.x > mid_x && self.pos.y < mid_y {
            return Some(Quadrant::Q2);
        }

        //Q3
        if self.pos.x < mid_x && self.pos.y > mid_y {
            return Some(Quadrant::Q3);
        }

        //Q4
        if self.pos.x > mid_x && self.pos.y > mid_y {
            return Some(Quadrant::Q4);
        }

        panic!("Cannot determine what quadrant robot should be in.")
    }
}

pub fn parse_input(input: String) -> Vec<Robot> {
    let mut res = vec![];
    for line in input.lines() {
        let mut rob = Robot {
            pos: Point { x: 0, y: 0 },
            vel: Point { x: 0, y: 0 },
        };

        let chars: Vec<char> = line.chars().collect();
        let mut idx = 0;

        while idx < chars.len() {
            if chars[idx] == 'p' {
                //parse positions
                //skip p and =
                idx += 2;
                let mut sign = 1;
                let mut curr_num: i64 = 0;

                while idx < chars.len() {
                    //check for negative sign
                    if chars[idx] == '-' {
                        sign = -1;
                    } else if chars[idx] == ',' {
                        rob.pos.x = curr_num * sign;
                        //reset;
                        curr_num = 0;
                        sign = 1;
                    } else if chars[idx].is_whitespace() {
                        rob.pos.y = curr_num * sign;
                        //exit loop (found end of second number)
                        break;
                    } else if chars[idx].is_numeric() {
                        curr_num = (curr_num * 10) + chars[idx].to_digit(10).unwrap() as i64;
                    } else {
                        panic!("bad char:{} at idx: {}", chars[idx], idx)
                    }
                    idx += 1;
                }
            }

            if chars[idx] == 'v' {
                //parse velocities

                //skip v and =
                idx += 2;
                let mut sign = 1;
                let mut curr_num: i64 = 0;

                while idx < chars.len() {
                    //check for negative sign
                    if chars[idx] == '-' {
                        sign = -1;
                    } else if chars[idx] == ',' {
                        rob.vel.x = curr_num * sign;
                        //reset;
                        curr_num = 0;
                        sign = 1;
                    } else if chars[idx].is_numeric() {
                        curr_num = (curr_num * 10) + chars[idx].to_digit(10).unwrap() as i64;
                    } else {
                        panic!("bad char:{} at idx: {}", chars[idx], idx)
                    }

                    if idx == chars.len() - 1 {
                        rob.vel.y = curr_num * sign;
                        //exit loop (found end of second number)
                        break;
                    }
                    idx += 1;
                }
            }
            idx += 1;
        }
        res.push(rob);
    }

    res
}
