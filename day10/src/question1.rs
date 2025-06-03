use std::collections::HashSet;

use crate::util::{Point, create_map};

use super::util;

fn is_in_bounds(map: &Vec<Vec<usize>>, pt: &Point) -> bool {
    if pt.y < map.len() && pt.x < map[pt.y].len() {
        return true;
    }
    false
}

fn is_ascending(map: &Vec<Vec<usize>>, pt_1: &Point, pt_2: &Point) -> bool {
    let val_1 = map[pt_1.y][pt_1.x];
    let val_2 = map[pt_2.y][pt_2.x];
    if val_1 < val_2 && val_2 - val_1 == 1 {
        return true;
    }
    false
}

fn calc_trail_ends(
    map: &Vec<Vec<usize>>,
    // visited: &Vec<Vec<bool>>,
    curr_pt: &Point,
    ends: &mut Vec<Point>,
) {
    //check up
    let up_pt = Point {
        x: curr_pt.x,
        y: curr_pt.y + 1,
    };
    if is_in_bounds(map, &up_pt) && is_ascending(map, &curr_pt, &up_pt) {
        calc_trail_ends(map, &up_pt, ends);
    }

    //check right
    let rt_pt = Point {
        x: curr_pt.x + 1,
        y: curr_pt.y,
    };
    if is_in_bounds(map, &rt_pt) && is_ascending(map, &curr_pt, &rt_pt) {
        calc_trail_ends(map, &rt_pt, ends);
    }

    //check down
    if curr_pt.y > 0 {
        let dn_pt = Point {
            x: curr_pt.x,
            y: curr_pt.y - 1,
        };
        if is_in_bounds(map, &dn_pt) && is_ascending(map, &curr_pt, &dn_pt) {
            calc_trail_ends(map, &dn_pt, ends);
        }
    }

    //check left
    if curr_pt.x > 0 {
        let lt_pt = Point {
            x: curr_pt.x - 1,
            y: curr_pt.y,
        };
        if is_in_bounds(map, &lt_pt) && is_ascending(map, &curr_pt, &lt_pt) {
            calc_trail_ends(map, &lt_pt, ends);
        }
    }
     ends.push(*curr_pt);
}

pub fn solve(input: &Vec<&str>) -> usize {
    let mut scores = Vec::new();

    let map = create_map(&input);
    let trailheads = util::get_trailheads(&map);
    // let mut blank_visited = vec![];
    // map.iter()
    //     .for_each(|row| blank_visited.push(vec![false; row.len()]));

    for t in trailheads {
        // let mut visited = blank_visited.clone();

        let mut trail_ends = Vec::new();
        calc_trail_ends(&map, &t, &mut trail_ends);

        let unique_trail_ends: HashSet<Point> = trail_ends.iter().copied().collect();


        let trail_ends_max = unique_trail_ends.iter().filter(|te| map[te.y][te.x]== 9).count();
        scores.push(trail_ends_max);
    }

    scores.iter().sum()
}
