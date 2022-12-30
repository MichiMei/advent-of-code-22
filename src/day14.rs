#![allow(dead_code, unused_mut)]

use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let mut game = Game::from(&input);

    println!("{}", game);

    while game.spawn_sand() {
        println!("{}", game);
    }

    println!("{}", game.count_sand());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut game = Game::from(&input);

    //println!("{}", game);

    while game.spawn_sand_floor() {
        //println!("{}", game);
    }

    println!("{}", game.count_sand());
}

struct Game {
    board: Vec<Vec<Element>>,
    spawn: (usize, usize),
    max_x: usize,
    max_y: usize,

}

impl Game {
    pub fn from(input: &Vec<String>) -> Game {
        let mut min_x = 500;
        let mut max_x = 500;
        let mut min_y = 0;
        let mut max_y = 0;

        let mut spawn_x = 500;
        let mut spawn_y = 0;

        let mut formations = vec![];
        for line in input {
            let mut points = vec![];
            let coords = line.split(" -> ");
            for coord in coords {
                let mut int_str = coord.split(",");
                let x = int_str.next().unwrap().parse::<usize>().unwrap();
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                let y = int_str.next().unwrap().parse::<usize>().unwrap();
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                points.push((x,y));
            }
            formations.push(points)
        }

        // normalize
        /*for formation in formations.iter_mut() {
            for point in formation.iter_mut() {
                point.0 -= min_x;
                point.1 -= min_y;
            }
        }
        max_x -= min_x;
        max_y -= min_y;
        spawn_x -= min_x;
        spawn_y -= min_y;*/

        let mut matrix = vec![];
        //matrix.resize(max_x+1, vec![]);
        matrix.resize(max_x+200, vec![]);
        for row in matrix.iter_mut() {
            //row.resize(max_y+1, Element::Air);
            row.resize(max_y+2, Element::Air);
        }

        // set spawn
        matrix[spawn_x][spawn_y] = Element::Spawn;

        let mut game = Game{board: matrix, spawn: (spawn_x, spawn_y), max_x, max_y};
        //println!("{}", game);

        // set lines
        for formation in formations {
            for index in 1..formation.len() {
                game.set_rock(formation[index-1], formation[index]);
                //println!("{}", game);
            }
        }

        game
    }

    pub fn spawn_sand(&mut self) -> bool {
        assert!(self.board[self.spawn.0][self.spawn.1] == Element::Spawn);
        let mut sand = self.spawn.clone();

        loop {
            if sand.1+1 > self.max_y {
                return false
            } else if self.board[sand.0][sand.1+1] == Element::Air {
                sand.1 += 1;
                continue
            }
            if sand.1+1 > self.max_y || sand.0 == 0 {
                return false
            } else if self.board[sand.0-1][sand.1+1] == Element::Air {
                sand.0 -= 1;
                sand.1 += 1;
                continue
            }
            if sand.1+1 > self.max_y || sand.0+1 > self.max_x {
                return false
            } else if self.board[sand.0+1][sand.1+1] == Element::Air {
                sand.0 += 1;
                sand.1 += 1;
                continue
            }
            break
        }
        assert!(self.board[sand.0][sand.1] == Element::Air);
        self.board[sand.0][sand.1] = Element::Sand;
        true
    }

    pub fn spawn_sand_floor(&mut self) -> bool {
        if self.board[self.spawn.0][self.spawn.1] == Element::Sand {
            return false
        }

        let mut sand = self.spawn.clone();

        loop {
            if sand.1 == self.max_y+1 {
                break
            }

            if self.board[sand.0][sand.1+1] == Element::Air {
                sand.1 += 1;
                continue
            }
            if sand.0 == 0 {
                panic!()
            } else if self.board[sand.0-1][sand.1+1] == Element::Air {
                sand.0 -= 1;
                sand.1 += 1;
                continue
            }
            if sand.0+1 >= self.board.len() {
                panic!()
            } else if self.board[sand.0+1][sand.1+1] == Element::Air {
                sand.0 += 1;
                sand.1 += 1;
                continue
            }
            break
        }
        assert!(self.board[sand.0][sand.1] == Element::Air || self.board[sand.0][sand.1] == Element::Spawn);
        self.board[sand.0][sand.1] = Element::Sand;
        true
    }

    pub fn count_sand(&self) -> usize {
        let mut count = 0;
        for row in self.board.iter() {
            for elem in row.iter() {
                if elem == &Element::Sand {
                    count += 1;
                }
            }
        }
        count
    }

    fn set_rock(&mut self, p0: (usize, usize), p1: (usize, usize)) {
        println!("line from {:?} to {:?}", p0, p1);
        if p0.0 == p1.0 {
            self.set_vertical(p0.0, p0.1, p1.1);
        } else if p0.1 == p1.1 {
            self.set_horizontal(p0.1, p0.0, p1.0);
        } else {
            panic!()
        }
    }

    fn set_vertical(&mut self, x: usize, mut y0: usize, mut y1: usize) {
        println!("vertical line at {} from {} to {}", x, y0, y1);
        if y0 > y1 {
            let tmp = y1;
            y1 = y0;
            y0 = tmp;
        }
        for y in y0..=y1 {
            assert!(self.board[x][y] == Element::Air || self.board[x][y] == Element::Rock);
            self.board[x][y] = Element::Rock
        }
    }

    fn set_horizontal(&mut self, y: usize, mut x0: usize, mut x1: usize) {
        println!("horizontal line at {} from {} to {}", y, x0, x1);
        if x0 > x1 {
            let tmp = x1;
            x1 = x0;
            x0 = tmp;
        }
        for x in x0..=x1 {
            assert!(self.board[x][y] == Element::Air || self.board[x][y] == Element::Rock);
            self.board[x][y] = Element::Rock
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*for row in self.board.iter() {
            write!(f, "|").unwrap();
            for elem in row.iter() {
                write!(f, "{}", elem).unwrap();
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "\n")*/

        for x in 0..self.board[0].len() {
            write!(f, "|").unwrap();
            for y in 0..self.board.len() {
                write!(f, "{}", self.board[y][x]).unwrap();
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "\n")
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Element {
    Air,
    Rock,
    Sand,
    Spawn,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Air => write!(f, " "),
            Element::Rock => write!(f, "#"),
            Element::Sand => write!(f, "o"),
            Element::Spawn => write!(f, "+"),
        }
    }
}

