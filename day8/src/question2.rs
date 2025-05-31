use std::{collections::HashMap, ops::{Add, Sub}};

#[derive(Debug, PartialEq, PartialOrd)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct FreqNode {
    freq: char,
    coords: Coordinate,
}

impl Add for &Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
        }
    }
}

impl Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn get_freq_nodes(input: &Vec<&str>) -> HashMap<char, Vec<FreqNode>> {
    let mut freq_map = HashMap::<char, Vec<FreqNode>>::new();

    for line_num in 0..input.len() {
        for char_num in 0..input[line_num].len() {
            let current_char = input[line_num].chars().nth(char_num).unwrap();
            if current_char.is_alphanumeric() {
                let new_node = FreqNode {
                    freq: current_char,
                    coords: Coordinate {
                        x: char_num as i64,
                        y: line_num as i64,
                    },
                };
                //create vec in map if does not exist
                if !freq_map.contains_key(&current_char) {
                    freq_map.insert(current_char, vec![new_node]);
                } else {
                    let freq_vec = freq_map.get_mut(&current_char).unwrap();
                    freq_vec.push(new_node);
                }
            }
        }
    }

    return freq_map;
}

fn is_in_bounds(node_pos: &Coordinate, max_bounds: &Coordinate) -> bool {
    if node_pos.x >= 0 && node_pos.x < max_bounds.x && node_pos.y >= 0 && node_pos.y < max_bounds.y {
        return true;
    }
    false
}

pub fn question2(input : &Vec<&str>) -> i64 {
    let freq_map = get_freq_nodes(&input);

    let mut antinode_vec: Vec<Vec<i64>> = input
        .clone()
        .iter()
        .into_iter()
        .map(|s| {
            return vec![0; s.len()];
        })
        .collect();

    let max_y = input.len();
    let max_x = input[0].len();
    let max_bounds = Coordinate {
        x: max_x as i64,
        y: max_y as i64,
    };

        for (_freq_id, freqnode_vec) in freq_map {
        for node_1 in &freqnode_vec {
            for node_2 in &freqnode_vec {
                //dont compare to self
                if node_1 != node_2 {
                    let dist = &node_2.coords - &node_1.coords;
                    let mut antinode = &node_2.coords + &dist;
                    //mark node_2 as a known antinode
                    antinode_vec[node_2.coords.y as usize][node_2.coords.x as usize] = 1;
                    
                    while is_in_bounds(&antinode, &max_bounds) {
                        antinode_vec[antinode.y as usize][antinode.x as usize] = 1;
                        antinode = &antinode + &dist;
                    }
                }
            }
        }
    }

    for idx_0 in 0..input.len() {
        let mut line = String::from("");
        for idx_1 in 0..input[0].len() {
            if antinode_vec[idx_0][idx_1] == 1{
                line += "#";
            } else {
                line += &String::from(input[idx_0].chars().nth(idx_1).unwrap());
            }
        }
        println!("{}", line);
    }
    antinode_vec.iter().flatten().sum()

}