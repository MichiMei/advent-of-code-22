#![allow(dead_code)]
extern crate core;

//use std::cmp::max;
//use std::fmt::{Display, Formatter};

mod aoc2022;

mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod lib;

fn main() {
    day25::a()
    //day25::b()

    //test()
}

/*fn test() {
    let mut board = Board::new();
    println!("{}", board);

    println!("{:?}", rec(&mut board, 0, 0, 0))

}

pub fn rec(board: &mut Board, x: usize, y: usize, curr_max: usize) -> Option<usize> {
    if y == board.grid.len() {
        let tmp = board.count();
        if tmp > curr_max {
            println!("new max found {}", tmp);
            return Some(tmp)
        }
        return None
    }
    if x == board.grid[y].len() {
        return rec(board, 0, y+1, curr_max)
    }
    if board.grid[y][x] != Content::Empty {
        return rec(board, x+1, y, curr_max)
    }

    let mut new_max = curr_max;

    if board.place_2_2(x, y) {
        println!("{}", board);
        if let Some(tmp) = rec(board, x+1, y, new_max) {
            new_max = max(new_max, tmp);
        }
        board.remove_2_2(x, y);
    }

    if board.place_1_2_horizontal(x, y) {
        println!("{}", board);
        if let Some(tmp) = rec(board, x+1, y, new_max) {
            new_max = max(new_max, tmp);
        }
        board.remove_1_2_horizontal(x, y);
    }

    if board.place_1_2_vertical(x, y) {
        println!("{}", board);
        if let Some(tmp) = rec(board, x+1, y, new_max) {
            new_max = max(new_max, tmp);
        }
        board.remove_1_2_vertical(x, y);
    }

    if new_max > curr_max {
        Some(new_max)
    } else {
        None
    }
}

pub struct Board {
    grid: [[Content; 11]; 11]
}

impl Board {
    pub fn new() -> Self {
        let mut grid: [[Content; 11]; 11] = Default::default();
        grid[0][9] = Content::Blocked;
        grid[7][1] = Content::Blocked;
        grid[10][0] = Content::Blocked;
        Board{grid}
    }

    pub fn place_1_2_horizontal(&mut self, x: usize, y: usize) -> bool {
        if y >= self.grid.len() {
            return false;
        }
        if x+1 >= self.grid[y].len() {
            return false;
        }
        if self.grid[y][x] != Content::Empty {
            return false;
        }
        if self.grid[y][x+1] != Content::Empty {
            return false;
        }
        self.grid[y][x] = Content::Horizontal;
        self.grid[y][x+1] = Content::Horizontal;
        true
    }

    pub fn remove_1_2_horizontal(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Content::Empty;
        self.grid[y][x+1] = Content::Empty;
    }

    pub fn place_1_2_vertical(&mut self, x: usize, y: usize) -> bool {
        if y+1 >= self.grid.len() {
            return false;
        }
        if x >= self.grid[y].len() {
            return false;
        }
        if self.grid[y][x] != Content::Empty {
            return false;
        }
        if self.grid[y+1][x] != Content::Empty {
            return false;
        }
        self.grid[y][x] = Content::Vertical;
        self.grid[y+1][x] = Content::Vertical;
        true
    }

    pub fn remove_1_2_vertical(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Content::Empty;
        self.grid[y+1][x] = Content::Empty;
    }

    pub fn place_2_2(&mut self, x: usize, y: usize) -> bool {
        if y+1 >= self.grid.len() {
            return false;
        }
        if x+1 >= self.grid[y].len() {
            return false;
        }
        if self.grid[y][x] != Content::Empty {
            return false;
        }
        if self.grid[y+1][x] != Content::Empty {
            return false;
        }
        if self.grid[y][x+1] != Content::Empty {
            return false;
        }
        if self.grid[y+1][x+1] != Content::Empty {
            return false;
        }
        self.grid[y][x] = Content::Big;
        self.grid[y+1][x] = Content::Big;
        self.grid[y][x+1] = Content::Big;
        self.grid[y+1][x+1] = Content::Big;
        true
    }

    pub fn remove_2_2(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Content::Empty;
        self.grid[y+1][x] = Content::Empty;
        self.grid[y][x+1] = Content::Empty;
        self.grid[y+1][x+1] = Content::Empty;
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for elem in row.iter() {
                if *elem == Content::Empty {
                    panic!()
                }
                if *elem == Content::Big {
                    count += 1;
                }
            }
        }
        assert_eq!(count % 4, 0);
        count/4
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            write!(f, "[").unwrap();
            for elem in line.iter() {
                write!(f, "{}", elem).unwrap();
            }
            write!(f, "]\n").unwrap();
        }
        write!(f, "\n")
    }
}

#[derive(Eq, PartialEq)]
pub enum Content {
    Empty,
    Blocked,
    Horizontal,
    Vertical,
    Big,
}

impl Default for Content {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Empty => write!(f, " "),
            Content::Blocked => write!(f, "X"),
            Content::Horizontal => write!(f, "-"),
            Content::Vertical => write!(f, "|"),
            Content::Big => write!(f, "O"),
        }
    }
}*/


/*fn test() {
    let source = [500, 400, 900];
    let dest   = [100, 450, 300, 400, 200, 350];

    let res = rec(source, dest, 0, usize::MAX);
    println!("{}", res);
}

fn rec(source: [usize; 3], dest: [usize; 6], sum: usize, current_opt: usize) -> usize {
    if sum > current_opt {
        return current_opt;
    }
    let mut s_i = usize::MAX;
    for (i, s) in source.iter().enumerate() {
        if *s != 0usize {
            s_i = i;
            break;
        }
    }
    if s_i == usize::MAX {
        return min(sum, current_opt);
    }

    let mut res = usize::MAX;

    for d_i in 0..dest.len() {
        if dest[d_i] == 0 {
            continue;
        }
        if MATRIX[s_i][d_i].is_none() {
            continue;
        }
        let transfer = min(source[s_i], dest[d_i]);
        let new_sum = sum + transfer* MATRIX[s_i][d_i].unwrap();
        let mut new_source = source.clone();
        new_source[s_i] -= transfer;
        let mut new_dest = dest.clone();
        new_dest[d_i] -= transfer;
        let tmp = rec(new_source, new_dest, new_sum, current_opt);
        res = min(res, tmp);
    }

    min(res, current_opt)
}

const MATRIX: [[Option<usize>; 6]; 3] = [   [Some(2), Some(4), None,    None,     Some(5), Some(8)],
                                            [Some(7), Some(8), Some(8), Some(11), Some(3), Some(5)],
                                            [Some(8), Some(6), Some(5), Some(4),  Some(9), Some(9)]];
 */