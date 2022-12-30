#![allow(dead_code)]

use std::cmp::max;
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let mut board = Board::from(input.first().unwrap());

    for _ in 0..2022 {
        //println!("{}", board);
        //println!("{}", board.get_height());
        board.rock_cycle(false);
    }
    //println!("{}", board);
    println!("{}", board.get_height());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut board = Board::from(input.first().unwrap());

    for index in 0..2500 {
        if board.rock_index == 0 {
            println!("{} jet index: {}", index, board.jet.position);
        }
        //println!("{}", board);
        //println!("{}", board.get_height());
        //if index%40 == 0 {
        //    board.rock_cycle(true);
        //} else {
            board.rock_cycle(false);
        //}

        /*if index%1000000==0 {
            let percent = (index as f64)/1000000000000f64 * 100f64;
            println!("{:.4}% done", percent);
        }*/
    }
    //println!("{}", board);
    println!("{}", board.get_height());
}

pub struct Board {
    grid: Vec<Vec<Element>>,
    jet: Jet,
    height: usize,
    rock_index: usize,
}

impl Board {
    pub fn from(str: &str) -> Board {
        let grid = vec![];
        let jet = Jet::from(str);
        Board{grid, jet, height: 0, rock_index: 0}
    }

    pub fn rock_cycle(&mut self, verbose: bool) {
        // adjust grid height
        while self.grid.len() <= self.height+7 {
            self.add_row();
        }

        // spawn rock
        let mut rock = Rock::spawn(self.rock_index, self.height+3);
        self.rock_index = (self.rock_index+1)%5;

        rock.move_direction(self.jet.get_next());
        if verbose {println!("rock moved");}
        loop {
            if !self.can_rock_fall(&rock) {
                break;
            }
            rock.move_down();
            if verbose {println!("rock fell");}
            let direction = self.jet.get_next();
            if self.can_rock_move(&rock, &direction) {
                rock.move_direction(direction);
            }
                if verbose { println!("rock moved");}
        }
    }

    fn can_rock_fall(&mut self, rock: &Rock) -> bool {
        for point in rock.points.iter() {
            if point.y == 0 || self.grid[point.y-1][point.x] == Element::Rock {
                self.solidify_rock(rock);
                return false
            }
        }
        true
    }

    fn can_rock_move(&mut self, rock: &Rock, direction: &Direction) -> bool {
        for point in rock.points.iter() {
            match direction {
                Direction::Right => {
                    if point.x == 6 {
                        return false
                    }
                    if self.grid[point.y][point.x+1] == Element::Rock {
                        return false
                    }
                }
                Direction::Left => {
                    if point.x == 0 {
                        return false
                    }
                    if self.grid[point.y][point.x-1] == Element::Rock {
                        return false
                    }
                }
            }
        }
        true
    }

    fn solidify_rock(&mut self, rock: &Rock) {
        for point in rock.points.iter() {
            assert!(self.grid[point.y][point.x] == Element::Air);
            self.grid[point.y][point.x] = Element::Rock;
            self.height = max(self.height, point.y+1);
        }
    }

    fn add_row(&mut self) {
        let row = vec![Element::Air; 7];
        self.grid.push(row);
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter().rev() {
            write!(f, "|").unwrap();
            for elem in row.iter() {
                write!(f, "{}", elem).unwrap();
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "\n")
    }
}

pub struct Jet {
    directions: String,
    length: usize,
    position: usize,
}

impl Jet {
    pub fn from(str: &str) -> Jet {
        let directions = String::from(str);
        let length = directions.as_bytes().len();
        let position = 0;
        Jet{directions, length, position}
    }

    pub fn get_next(&mut self) -> Direction {
        let next = self.directions.as_bytes()[self.position] as char;
        self.position = (self.position+1)%self.length;
        match next {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!()
        }
    }
}

pub struct Rock {
    points: Vec<Point>,
}

impl Rock {
    pub fn spawn(index: usize, y: usize) -> Rock {
        match index {
            0 => Rock::create_0(y),
            1 => Rock::create_1(y),
            2 => Rock::create_2(y),
            3 => Rock::create_3(y),
            4 => Rock::create_4(y),
            _ => panic!(),
        }
    }

    pub fn move_down(&mut self) {
        for point in self.points.iter_mut() {
            point.y -= 1;
        }
    }

    pub fn move_direction(&mut self, direction: Direction) -> bool {
        let mut movable = true;
        for point in self.points.iter() {
            match direction {
                Direction::Right => {
                    if point.x == 6 {
                        movable = false;
                        break;
                    }
                }
                Direction::Left => {
                    if point.x == 0 {
                        movable = false;
                        break;
                    }
                }
            }
        }
        if !movable {
            return false
        }
        for point in self.points.iter_mut() {
            match direction {
                Direction::Right => point.x += 1,
                Direction::Left => point.x -= 1,
            }
        }
        true
    }

    fn create_0(y: usize) -> Self {
        let mut points = vec![];
        points.push(Point{x: 2, y});
        points.push(Point{x: 3, y});
        points.push(Point{x: 4, y});
        points.push(Point{x: 5, y});
        Rock{points}
    }

    fn create_1(y: usize) -> Self {
        let mut points = vec![];
        points.push(Point{x: 3, y});
        points.push(Point{x: 2, y: y+1});
        points.push(Point{x: 3, y: y+1});
        points.push(Point{x: 4, y: y+1});
        points.push(Point{x: 3, y: y+2});
        Rock{points}
    }

    fn create_2(y: usize) -> Self {
        let mut points = vec![];
        points.push(Point{x: 2, y});
        points.push(Point{x: 3, y});
        points.push(Point{x: 4, y});
        points.push(Point{x: 4, y: y+1});
        points.push(Point{x: 4, y: y+2});
        Rock{points}
    }

    fn create_3(y: usize) -> Self {
        let mut points = vec![];
        points.push(Point{x: 2, y});
        points.push(Point{x: 2, y: y+1});
        points.push(Point{x: 2, y: y+2});
        points.push(Point{x: 2, y: y+3});
        Rock{points}
    }

    fn create_4(y: usize) -> Self {
        let mut points = vec![];
        points.push(Point{x: 2, y});
        points.push(Point{x: 3, y});
        points.push(Point{x: 2, y: y+1});
        points.push(Point{x: 3, y: y+1});
        Rock{points}
    }
}

pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Element {
    Air,
    Rock,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Air => write!(f, "."),
            Element::Rock => write!(f, "#"),
        }
    }
}

pub enum Direction {
    Right,
    Left,
}