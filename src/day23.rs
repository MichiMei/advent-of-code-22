#![allow(dead_code)]

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut locations = Locations::from(&input);
    println!("{}", locations);
    for _ in 0..10 {
        locations.run_round();
        println!("{}", locations);
    }
    println!("{}", locations.get_free_count());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut locations = Locations::from(&input);
    let mut round = 1;
    loop {
        if !locations.run_round() {
            break
        }
        round += 1;
        if round%100 == 0 {
            println!("{}", round);
        }
    }
    println!("{}", round);
}

pub struct Locations {
    locations: HashSet<Point>,
    start_dir: Direction,
}

impl Locations {
    pub fn from(input: &Vec<String>) -> Self {
        let mut locations = HashSet::new();
        for (row_index, row) in input.iter().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                if col == '#' {
                    let x = col_index as i32;
                    let y = row_index as i32;
                    locations.insert(Point{x, y});
                }
            }
        }
        let start_dir = Direction::North;
        Self{locations, start_dir}
    }

    pub fn run_round(&mut self) -> bool {
        let proposed = self.propose();
        if proposed.is_empty() {
            return false
        }
        self.move_proposals(proposed);
        self.start_dir = self.start_dir.next();
        true
    }

    pub fn get_free_count(&self) -> usize {
        let (min, max) = self.get_limits();
        let y = max.y - min.y + 1;
        assert!(y > 0);
        let x = max.x - min.x + 1;
        assert!(x > 0);
        (y as usize)*(x as usize) - self.locations.len()
    }

    fn propose(&self) -> HashMap<Point, Option<Point>> {
        let mut proposed = HashMap::new();
        for elf in self.locations.iter() {
            if let Some(proposal) = self.propose_elf(*elf) {
                if let Some(_) = proposed.get(&proposal) {
                    proposed.insert(proposal, None);
                } else {
                    proposed.insert(proposal, Some(*elf));
                }
            }
        }
        proposed
    }

    fn move_proposals(&mut self, proposed: HashMap<Point, Option<Point>>) {
        for (dest, source_option) in proposed {
            if let Some(source) = source_option {
                assert!(self.locations.remove(&source));
                assert!(self.locations.insert(dest));
            }
        }
    }

    fn propose_elf(&self, elf: Point) -> Option<Point> {
        let mut direction = self.start_dir;
        let mut free_count = 0;
        let mut free_point = None;
        for _ in 0..4 {
            let relevant = direction.get_relevant_points(elf);
            let mut is_free = true;
            for p in relevant {
                if !self.check_point(p) {
                    is_free = false;
                    break;
                }
            }

            if is_free {
                free_count += 1;
                if free_point.is_none() {
                    free_point = Some(direction.get_move_point(elf));
                }
            }
            direction = direction.next();
        }
        if free_count == 4 {
            return None;
        }
        free_point
    }

    fn check_point(&self, p: Point) -> bool {
        if self.locations.contains(&p) {
            return false
        }
        true
    }

    fn get_limits(&self) -> (Point, Point) {
        let mut maximum = Point{x: i32::MIN, y: i32::MIN};
        let mut minimum = Point{x: i32::MAX, y: i32::MAX};
        for loc in self.locations.iter() {
            maximum.x = max(maximum.x, loc.x);
            maximum.y = max(maximum.y, loc.y);
            minimum.x = min(minimum.x, loc.x);
            minimum.y = min(minimum.y, loc.y);
        }
        (minimum, maximum)
    }

    fn to_string(&self) -> String {
        let (min, max) = self.get_limits();
        let col_count = max.x - min.x + 1;
        let row_count = max.y - min.y + 1;

        let mut bytes = vec![];
        bytes.resize(row_count as usize, vec![]);
        for line in bytes.iter_mut() {
            line.resize(col_count as usize, '.' as u8);
        }

        for loc in self.locations.iter() {
            let row = loc.y - min.y;
            assert!(row >= 0 && (row as usize) < bytes.len());
            let col = loc.x - min.x;
            assert!(col >= 0 && (col as usize) < bytes[row as usize].len());

            bytes[row as usize][col as usize] = '#' as u8;
        }

        let mut res = String::new();
        for line in bytes {
            res = format!("{}\n{}", res, String::from_utf8(line).unwrap());
        }
        res
    }
}

impl Display for Locations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn get_relevant_points(&self, p: Point) -> [Point; 3] {
        match self {
            Direction::North => {
                [
                    Point{ x: p.x-1,   y: p.y-1 },
                    Point{ x: p.x  ,   y: p.y-1 },
                    Point{ x: p.x+1,   y: p.y-1 },
                ]
            }
            Direction::South => {
                [
                    Point{ x: p.x-1,   y: p.y+1 },
                    Point{ x: p.x  ,   y: p.y+1 },
                    Point{ x: p.x+1,   y: p.y+1 },
                ]
            }
            Direction::West => {
                [
                    Point{ x: p.x-1,   y: p.y-1 },
                    Point{ x: p.x-1,   y: p.y   },
                    Point{ x: p.x-1,   y: p.y+1 },
                ]
            }
            Direction::East => {
                [
                    Point{ x: p.x+1,   y: p.y-1 },
                    Point{ x: p.x+1,   y: p.y   },
                    Point{ x: p.x+1,   y: p.y+1 },
                ]
            }
        }
    }

    pub fn get_move_point(&self, p: Point) -> Point {
        match self {
            Direction::North => {
                Point{ x: p.x,   y: p.y-1 }
            }
            Direction::South => {
                Point{ x: p.x,   y: p.y+1 }
            }
            Direction::West => {
                Point{ x: p.x-1,   y: p.y }
            }
            Direction::East => {
                Point{ x: p.x+1,   y: p.y }
            }
        }
    }
}