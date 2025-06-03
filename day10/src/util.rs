#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}


pub fn create_map(input: &Vec<&str>) -> Vec<Vec<usize>>{
    let mut res = Vec::new();

    for line in input {
        let line_vec = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        res.push(line_vec);
    }
    res
}

pub fn get_trailheads(input: &Vec<Vec<usize>>) -> Vec<Point> {
    let mut res = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 0 {
                res.push(Point {x: x, y: y});
            }
        }
    }


    res
}
