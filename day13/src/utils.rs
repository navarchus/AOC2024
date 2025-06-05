use regex::Regex;
use std::{
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug,Clone,Copy)]
pub struct ClawMachine {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn parse_claw_machines(input: &String) -> Vec<ClawMachine> {
    let mut res = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let val_regex = Regex::new(r"X[\+=]{1}(?<xval>[0-9]+), Y[\+=]{1}(?<yval>[0-9]+)").unwrap();

    let mut idx = 0;
    while idx < lines.len() {
        let button_a_vals = val_regex.captures(lines[idx]).unwrap();
        let button_b_vals = val_regex.captures(lines[idx + 1]).unwrap();
        let prize_vals = val_regex.captures(lines[idx + 2]).unwrap();

        res.push(ClawMachine {
            button_a: Point {
                x: button_a_vals["xval"].parse().unwrap(),
                y: button_a_vals["yval"].parse().unwrap(),
            },
            button_b: Point {
                x: button_b_vals["xval"].parse().unwrap(),
                y: button_b_vals["yval"].parse().unwrap(),
            },
            prize: Point {
                x: prize_vals["xval"].parse().unwrap(),
                y: prize_vals["yval"].parse().unwrap(),
            },
        });

        //one line is blank in each 'block' of claw machine info, skip 4 lines total
        idx += 4;
    }

    res
}

impl ClawMachine {
    pub fn solve_machine_tokens(&self) -> i64 {
        let b = (self.prize.y*self.button_a.x - self.prize.x*self.button_a.y) / (self.button_a.x*self.button_b.y - self.button_a.y*self.button_b.x);
        let a = (self.prize.y - (b*self.button_b.y))/self.button_a.y;

        if (a*self.button_a.x + b*self.button_b.x) != self.prize.x || (a*self.button_a.y + b*self.button_b.y) != self.prize.y  {
            return 0;
        }

        3*a + b
    }
}
