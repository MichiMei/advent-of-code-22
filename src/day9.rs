#![allow(dead_code)]

use std::borrow::BorrowMut;
use std::collections::HashSet;

pub fn a() {
    let input = advent_of_code_22::read_lines();

    //let mut game = Game::new();
    let mut game = Game::new(2);

    for line in input {
        let (dir, steps) = parse_command(&line);
        game.handle_command(dir, steps);
    }

    println!("{}", game.visited.len());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut game = Game::new(10);

    for line in input {
        let (dir, steps) = parse_command(&line);
        game.handle_command(dir, steps);
    }

    println!("{}", game.visited.len());
}

fn parse_command(command: &str) -> (Direction, usize) {
    let mut words = command.split(" ");
    let dir_str = words.next().unwrap();
    let steps_str = words.next().unwrap();
    let dir = Direction::from(dir_str);
    let steps = steps_str.parse::<usize>().unwrap();

    (dir, steps)
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn from(str: &str) -> Self {
        assert_eq!(str.len(), 1);

        return match str.chars().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ =>  panic!(),
        }
    }
}

struct Game {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Game {
    pub fn new(count: usize) -> Self {
        let mut knots = vec![];
        for _ in 0..count {
            let knot = Point::new(0,0);
            knots.push(knot);
        }
        let mut visited = HashSet::new();
        visited.insert(knots.last().unwrap().clone());
        Game { knots, visited }
    }

    pub fn handle_command(&mut self, dir: Direction, steps: usize) {
        for _ in 0..steps {
            self.move_head(dir);
            self.follow_tails();
        }
    }

    fn move_head(&mut self, dir: Direction) {
        let head = self.knots.first_mut().unwrap();
        match dir {
            Direction::Right => head.x += 1,
            Direction::Left => head.x -= 1,
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
        }
    }

    fn follow_tails(&mut self) {
        for index in 1..self.knots.len() {
            let (head_slice, tail_slice) = self.knots.split_at_mut(index);
            if !head_slice.last().unwrap().touching(&tail_slice.first().unwrap()) {
                tail_slice.first_mut().unwrap().follow(&head_slice.last().unwrap());
            }
        }
        self.add_tail_to_set();
    }

    fn add_tail_to_set(&mut self) {
        let last = self.knots.last().unwrap().clone();
        println!("added \t{}\t{}", last.x, last.y);
        self.visited.borrow_mut().insert(last);
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{x,y}
    }

    pub fn touching(&self, other: &Point) -> bool {
        if self.x.abs_diff(other.x) <= 1 &&
            self.y.abs_diff(other.y) <= 1 {
            return true
        }
        false
    }

    pub fn follow(&mut self, target: &Point) {
        Self::follow_coord(&mut self.x, &target.x);
        Self::follow_coord(&mut self.y, &target.y);
    }

    fn follow_coord(main: &mut i32, target: &i32) {
        match main.abs_diff(*target) {
            0 => {}
            1 => *main = *target,
            2 => {
                let value = *target-*main;
                *main += value/value.abs();
            }
            _ => panic!(),
        }
    }
}