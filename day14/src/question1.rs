use crate::utils::Quadrant;

use super::utils::Robot;

pub fn solve(robots: &Vec<Robot>, bounds: (usize, usize), steps: usize) -> usize {
    let mut bots = robots.clone();

    for _ in 0..steps {
        bots = bots.iter().map(|b: &Robot| b.step(bounds)).collect();
    }

    bots.iter().filter(|b| b.get_quadrant(bounds).is_some() && b.get_quadrant(bounds).unwrap() == Quadrant::Q1).count() *
    bots.iter().filter(|b| b.get_quadrant(bounds).is_some() && b.get_quadrant(bounds).unwrap() == Quadrant::Q2).count() *
    bots.iter().filter(|b| b.get_quadrant(bounds).is_some() && b.get_quadrant(bounds).unwrap() == Quadrant::Q3).count() *
    bots.iter().filter(|b| b.get_quadrant(bounds).is_some() && b.get_quadrant(bounds).unwrap() == Quadrant::Q4).count()

}