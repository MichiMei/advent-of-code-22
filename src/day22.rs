#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;

pub fn a() {
    let input = advent_of_code_22::read_lines_untrimmed();
    let mut board = Board::from(&input[..input.len()-2]);
    let movement_string = &input[input.len()-1];
    let mut chars = movement_string.chars().peekable();
    println!("{} {}", board.position, board.direction);
    loop {
        // read steps
        let mut steps = 0;
        while chars.peek().is_some() && chars.peek().unwrap().is_digit(10) {
            let next = String::from(chars.next().unwrap()).parse::<usize>().unwrap();
            assert!(next < 10);
            steps = steps*10+next;
        }
        board.move_tiles(steps);
        println!("{} {}", board.position, board.direction);
        // read direction
        if chars.peek().is_none() {
            break;
        }
        let c = chars.next().unwrap();
        board.change_direction(c);
        println!("{} {}", board.position, board.direction);
    }
    println!("{} {}", board.position, board.direction);
    println!("{}", board.get_position_score())
}

//const SIDE_LENGTH: usize = 4;
const SIDE_LENGTH: usize = 50;

const TOP: usize = 0;
const FRONT: usize = 1;
const BOT: usize = 2;
const BACK: usize = 3;
const RIGHT: usize = 4;
const LEFT: usize = 5;

pub fn b() {
    /*let positions = [
        (0, 2),
        (1, 2),
        (2, 2),
        (1, 0),
        (2, 3),
        (1, 1),
    ];
    let orientations = [
        SideOrientation::Turned00,
        SideOrientation::Turned00,
        SideOrientation::Turned00,
        SideOrientation::Turned18,
        SideOrientation::Turned18,
        SideOrientation::Turned27,
    ];*/

    let positions = [
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 0),
        (0, 2),
        (2, 0),
    ];
    let orientations = [
        SideOrientation::Turned00,
        SideOrientation::Turned00,
        SideOrientation::Turned00,
        SideOrientation::Turned09,
        SideOrientation::Turned00,
        SideOrientation::Turned18,
    ];

    let input = advent_of_code_22::read_lines_untrimmed();
    let mut cube = Cube::from(&input[0..input.len()-2], positions, orientations);
    println!("{}", cube);
    cube.movement(&input[input.len()-1], false);
    cube.print_pos();
}

pub struct Cube {
    sides: [Side; 6],
    curr_side: usize,
    side_pos: (usize, usize),
    direction: CubeDir,
}

type Position = (usize, usize);

impl Cube {
    pub fn from(input: &[String], positions: [(usize, usize); 6], orientations: [SideOrientation; 6]) -> Self {
        let mut sides: [Side; 6] = Default::default();

        for index in 0..sides.len() {
            sides[index] = Side::parse_side(input, positions[index], orientations[index]);
        }

        Self{
            sides,
            curr_side: 0,
            side_pos: (0, 0),
            direction: CubeDir::East,
        }
    }

    pub fn movement(&mut self, str: &str, verbose: bool) {
        let mut iter = str.chars().peekable();
        while iter.peek().is_some() {
            self.move_pos(&mut iter, verbose);
            if iter.peek().is_none() {
                break;
            }
            self.change_dir(&mut iter, verbose);
        }
    }

    pub fn print_pos(&self) {
        println!("side: {}", self.curr_side);
        println!("pos: {:?}", self.side_pos);
        println!("dir: {:?}", self.direction);
    }

    fn move_pos(&mut self, iter: &mut Peekable<Chars>, verbose: bool) {
        if verbose {
            println!("move");
        }
        let mut int = 0;
        while iter.peek().is_some() && iter.peek().unwrap().is_digit(10) {
            let char = iter.next().unwrap();
            let digit = String::from(char).parse::<usize>().unwrap();
            assert!(digit < 10);
            int = int*10 + digit;
        }
        while int > 0 {
            match self.sides[self.curr_side].move_step(self.side_pos, self.direction) {
                Ok(Some(new_pos)) => {
                    self.side_pos = new_pos;
                    if verbose {
                        println!("{}", self);
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(exit_dir) => {
                    if verbose {
                        println!("switching side from {} at {:?} facing {:?}", self.curr_side, self.side_pos, self.direction);
                    }
                    self.switch_side(exit_dir, verbose);
                    self.sides[self.curr_side].set_direction(self.side_pos, self.direction);
                    if verbose {
                        println!("to {} at {:?} facing {:?}", self.curr_side, self.side_pos, self.direction);
                        println!("{}", self);
                    }
                }
            }
            int -= 1;
        }
    }

    fn change_dir(&mut self, iter: &mut Peekable<Chars>, verbose: bool) {
        if verbose {
            println!("change direction");
        }
        self.direction = self.direction.change_direction(iter.next().unwrap());
        self.sides[self.curr_side].set_direction(self.side_pos, self.direction);
        if verbose {
            println!("{}", self);
        }
    }

    fn switch_side(&mut self, dir: CubeDir, verbose: bool) -> bool {
        if verbose {
            println!("switch side");
        }
        match self.curr_side {
            0 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 3;
                        let new_pos = (SIDE_LENGTH - 1, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 4;
                        let new_pos = (self.side_pos.0, 0);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 1;
                        let new_pos = (0, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 5;
                        let new_pos = (self.side_pos.0, SIDE_LENGTH - 1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            1 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 0;
                        let new_pos = (SIDE_LENGTH - 1, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 4;
                        let new_pos = (SIDE_LENGTH - 1, self.side_pos.0);
                        let new_dir = CubeDir::North;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 2;
                        let new_pos = (0, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 5;
                        let new_pos = (SIDE_LENGTH - 1, SIDE_LENGTH - self.side_pos.0 - 1);
                        let new_dir = CubeDir::North;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            2 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 1;
                        let new_pos = (SIDE_LENGTH - 1, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 4;
                        let new_pos = (SIDE_LENGTH - self.side_pos.0-1, SIDE_LENGTH - 1);
                        let new_dir = CubeDir::West;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 3;
                        let new_pos = (0, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 5;
                        let new_pos = (SIDE_LENGTH - self.side_pos.0 - 1, 0);
                        let new_dir = CubeDir::East;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            3 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 2;
                        let new_pos = (SIDE_LENGTH - 1, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 4;
                        let new_pos = (0, SIDE_LENGTH - self.side_pos.0 - 1);
                        let new_dir = CubeDir::South;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 0;
                        let new_pos = (0, self.side_pos.1);
                        let new_dir = self.direction;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 5;
                        let new_pos = (0, self.side_pos.0);
                        let new_dir = CubeDir::South;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            4 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 3;
                        let new_pos = (SIDE_LENGTH - self.side_pos.1 - 1, SIDE_LENGTH - 1);
                        let new_dir = CubeDir::West;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 2;
                        let new_pos = (SIDE_LENGTH - self.side_pos.0 - 1, SIDE_LENGTH - 1);
                        let new_dir = CubeDir::West;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 1;
                        let new_pos = (self.side_pos.1, SIDE_LENGTH - 1);
                        let new_dir = CubeDir::West;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 0;
                        let new_pos = (self.side_pos.0, SIDE_LENGTH - 1);
                        let new_dir = CubeDir::West;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            5 => {
                return match dir {
                    CubeDir::North => {
                        let new_side = 3;
                        let new_pos = (self.side_pos.1, 0);
                        let new_dir = CubeDir::East;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::East => {
                        let new_side = 0;
                        let new_pos = (self.side_pos.0, 0);
                        let new_dir = CubeDir::East;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::South => {
                        let new_side = 1;
                        let new_pos = (SIDE_LENGTH - self.side_pos.1 - 1, 0);
                        let new_dir = CubeDir::East;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                    CubeDir::West => {
                        let new_side = 2;
                        let new_pos = (SIDE_LENGTH - self.side_pos.0 - 1, 0);
                        let new_dir = CubeDir::East;
                        if self.sides[new_side].is_free(new_pos) {
                            self.curr_side = new_side;
                            self.side_pos = new_pos;
                            self.direction = new_dir;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            _ => panic!(),
        }
    }

    fn to_string(&self) -> String {
        let mut str = String::new();

        for index in 0..SIDE_LENGTH {
            let s0 = self.sides[5].to_string_row(index);
            let s1 = self.sides[0].to_string_row(index);
            let s2 = self.sides[4].to_string_row(index);
            str = format!("{}   {}   {}   {}\n", str, s0, s1, s2);
        }
        str = format!("{}\n", str);

        for index in 0..SIDE_LENGTH {
            let fill = String::from(" ").repeat(SIDE_LENGTH);
            let s0 = self.sides[1].to_string_row(index);
            str = format!("{}   {}   {}   {}\n", str, fill, s0, fill);
        }
        str = format!("{}\n", str);

        for index in 0..SIDE_LENGTH {
            let fill = String::from(" ").repeat(SIDE_LENGTH);
            let s0 = self.sides[2].to_string_row(index);
            str = format!("{}   {}   {}   {}\n", str, fill, s0, fill);
        }
        str = format!("{}\n", str);

        for index in 0..SIDE_LENGTH {
            let fill = String::from(" ").repeat(SIDE_LENGTH);
            let s0 = self.sides[3].to_string_row(index);
            str = format!("{}   {}   {}   {}\n", str, fill, s0, fill);
        }
        str = format!("{}\n", str);

        str
    }

}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Side {
    grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH],
}

impl Side {
    pub fn parse_side(input: &[String], pos: (usize, usize), orientation: SideOrientation) -> Self {
        let start = (pos.0*SIDE_LENGTH, pos.1*SIDE_LENGTH);

        //let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = Default::default();
        const INIT_VAL: CubeFloor = CubeFloor::Wall;
        const INIT_ARRAY: [CubeFloor; SIDE_LENGTH] = [INIT_VAL; SIDE_LENGTH];
        let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = [INIT_ARRAY; SIDE_LENGTH];

        for (row_index, row) in (&input[start.0..start.0+SIDE_LENGTH]).iter().enumerate() {
            for (col_index, col) in (&row[start.1..start.1+SIDE_LENGTH]).chars().enumerate() {
                grid[row_index][col_index] = CubeFloor::from(col);
            }
        }
        let mut side = Self{grid};
        side.rotate(orientation);
        side
    }

    pub fn move_step(&mut self, pos: (usize, usize), dir: CubeDir) -> Result<Option<(usize, usize)>, CubeDir> {
        self.set_direction(pos, dir);
        match dir {
            CubeDir::North => {
                if pos.0 == 0 {
                    return Err(CubeDir::North)
                }
                let new_pos = (pos.0-1, pos.1);
                if self.grid[new_pos.0][new_pos.1] != CubeFloor::Wall {
                    self.set_direction(new_pos, dir);
                    Ok(Some(new_pos))
                } else {
                    Ok(None)
                }
            }
            CubeDir::East => {
                if pos.1+1 == self.grid[pos.0].len() {
                    return Err(CubeDir::East)
                }
                let new_pos = (pos.0, pos.1+1);
                if self.grid[new_pos.0][new_pos.1] != CubeFloor::Wall {
                    self.set_direction(new_pos, dir);
                    Ok(Some(new_pos))
                } else {
                    Ok(None)
                }
            }
            CubeDir::South => {
                if pos.0+1 == self.grid.len() {
                    return Err(CubeDir::South)
                }
                let new_pos = (pos.0+1, pos.1);
                if self.grid[new_pos.0][new_pos.1] != CubeFloor::Wall {
                    self.set_direction(new_pos, dir);
                    Ok(Some(new_pos))
                } else {
                    Ok(None)
                }
            }
            CubeDir::West => {
                if pos.1 == 0 {
                    return Err(CubeDir::West)
                }
                let new_pos = (pos.0, pos.1-1);
                if self.grid[new_pos.0][new_pos.1] != CubeFloor::Wall {
                    self.set_direction(new_pos, dir);
                    Ok(Some(new_pos))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub fn to_string_row(&self, index: usize) -> String {
        assert!(index < self.grid.len());
        let mut str = String::new();
        for x in self.grid[index].iter() {
            str = format!("{}{}", str, x);
        }
        str
    }

    pub fn rotate(&mut self, orientation: SideOrientation) {
        match orientation {
            SideOrientation::Turned00 => {}
            SideOrientation::Turned09 => self.rotate_90(),
            SideOrientation::Turned18 => self.rotate_180(),
            SideOrientation::Turned27 => self.rotate_270(),
        }
    }

    pub fn is_free(&self, pos: (usize, usize)) -> bool {
        if self.grid[pos.0][pos.1] == CubeFloor::Wall {
            return false
        }
        true
    }

    fn set_direction(&mut self, pos: (usize, usize), dir: CubeDir) {
        self.grid[pos.0][pos.1] = CubeFloor::from_cube_direction(dir);
    }

    fn rotate_90(&mut self) {
        //let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = Default::default();
        const INIT_VAL: CubeFloor = CubeFloor::Wall;
        const INIT_ARRAY: [CubeFloor; SIDE_LENGTH] = [INIT_VAL; SIDE_LENGTH];
        let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = [INIT_ARRAY; SIDE_LENGTH];

        for (x0, x1) in (0..SIDE_LENGTH).rev().enumerate() {
            for y in 0..SIDE_LENGTH {
                grid[x0][y] = self.grid[y][x1];
            }
        }
        self.grid = grid;
    }

    fn rotate_180(&mut self) {
        //let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = Default::default();
        const INIT_VAL: CubeFloor = CubeFloor::Wall;
        const INIT_ARRAY: [CubeFloor; SIDE_LENGTH] = [INIT_VAL; SIDE_LENGTH];
        let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = [INIT_ARRAY; SIDE_LENGTH];

        for (x0, x1) in (0..SIDE_LENGTH).rev().enumerate() {
            for (y0, y1) in (0..SIDE_LENGTH).rev().enumerate() {
                grid[x0][y0] = self.grid[x1][y1];
            }
        }
        self.grid = grid;
    }

    fn rotate_270(&mut self) {
        //let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = Default::default();
        const INIT_VAL: CubeFloor = CubeFloor::Wall;
        const INIT_ARRAY: [CubeFloor; SIDE_LENGTH] = [INIT_VAL; SIDE_LENGTH];
        let mut grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = [INIT_ARRAY; SIDE_LENGTH];

        for x in 0..SIDE_LENGTH {
            for (y0, y1) in (0..SIDE_LENGTH).rev().enumerate() {
                grid[x][y0] = self.grid[y1][x];
            }
        }
        self.grid = grid;
    }
}

impl Default for Side {
    fn default() -> Self {
        const INIT_VAL: CubeFloor = CubeFloor::Wall;
        const INIT_ARRAY: [CubeFloor; SIDE_LENGTH] = [INIT_VAL; SIDE_LENGTH];
        let grid: [[CubeFloor; SIDE_LENGTH]; SIDE_LENGTH] = [INIT_ARRAY; SIDE_LENGTH];
        Self{grid}
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum CubeFloor {
    Floor,
    Wall,
    Up,
    Right,
    Down,
    Left,
}

impl CubeFloor {
    pub fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            '#' => Self::Wall,
            _ => panic!(),
        }
    }

    pub fn from_cube_direction(dir: CubeDir) -> Self {
        match dir {
            CubeDir::North => Self::Up,
            CubeDir::East => Self::Right,
            CubeDir::South => Self::Down,
            CubeDir::West => Self::Left,
        }
    }
}

impl Default for CubeFloor {
    fn default() -> Self {
        Self::Wall
    }
}

impl Display for CubeFloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CubeFloor::Floor => write!(f, "."),
            CubeFloor::Wall => write!(f, "#"),
            CubeFloor::Up => write!(f, "^"),
            CubeFloor::Right => write!(f, ">"),
            CubeFloor::Down => write!(f, "v"),
            CubeFloor::Left => write!(f, "<"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CubeDir {
    North,
    East,
    South,
    West,
}

impl CubeDir {
    pub fn change_direction(self, c: char) -> Self {
        let index = self.get_index();
        match c {
            'L' => {
                if index == 0 {
                    Self::from_index(3)
                } else {
                    Self::from_index(index-1)
                }
            }
            'R' => {
                let new_index = (index+1)%4;
                Self::from_index(new_index)
            }
            _ => panic!(),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            CubeDir::East => 0,
            CubeDir::South => 1,
            CubeDir::West => 2,
            CubeDir::North => 3,
        }
    }

    /*pub fn get_local_direction(&self, ori: SideOrientation) -> CubeDir {
        let mut new_index = self.get_index();
        new_index = (new_index+ori.get_change())%4;
        Self::from_index(new_index)
    }*/

    /*pub fn get_global_direction(&self, ori: SideOrientation) -> CubeDir {
        let mut new_index = self.get_index();
        let change_index = 4-ori.get_change();
        new_index = (new_index+change_index)%4;
        Self::from_index(new_index)
    }*/

    fn from_index(index: usize) -> Self {
        assert!(index < 4);
        match index {
            0 => CubeDir::East,
            1 => CubeDir::South,
            2 => CubeDir::West,
            3 => CubeDir::North,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SideOrientation {
    Turned00,
    Turned09,
    Turned18,
    Turned27,
}

impl SideOrientation {
    pub fn get_change(&self) -> usize {
        match self {
            SideOrientation::Turned00 => 0,
            SideOrientation::Turned09 => 1,
            SideOrientation::Turned18 => 2,
            SideOrientation::Turned27 => 3,
        }
    }

    /*pub fn get_local_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            SideOrientation::Turned00 => pos,
            SideOrientation::Turned09 => (SIDE_LENGTH-pos.1, pos.0),
            SideOrientation::Turned18 => (SIDE_LENGTH-pos.0, SIDE_LENGTH-pos.1),
            SideOrientation::Turned27 => (pos.1, SIDE_LENGTH-pos.0),
        }
    }*/

    /*pub fn get_global_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            SideOrientation::Turned00 => pos,
            SideOrientation::Turned09 => (pos.1, SIDE_LENGTH-pos.0),
            SideOrientation::Turned18 => (SIDE_LENGTH-pos.0, SIDE_LENGTH-pos.1),
            SideOrientation::Turned27 => (SIDE_LENGTH-pos.1, pos.0),
        }
    }*/
}

impl Default for SideOrientation {
    fn default() -> Self {
        Self::Turned00
    }
}




pub struct Board {
    grid: Vec<Vec<Floor>>,
    position: PositionA,
    direction: Direction,
    modulo: PositionA,
}

impl Board {
    pub fn from(input: &[String]) -> Self {
        let mut grid = vec![];
        let mut start = None;
        let mut max = [input.len(), input[0].len()];
        grid.reserve(input.len());
        for (row_index, line) in input.iter().enumerate() {
            if line.len() > max[1] {
                max[1] = line.len();
            }
            let mut row = vec![];
            for (column_index, c) in line.chars().enumerate() {
                let tmp = Floor::from(c);
                if start.is_none() && tmp == Floor::Floor {
                    start = Some([row_index as i32, column_index as i32]);
                }
                row.push(tmp);
            }
            grid.push(row);
        }
        for line in grid.iter_mut() {
            line.resize(max[1], Floor::Nothing);
        }
        let position = PositionA(start.unwrap());
        let direction = Direction::East;
        let modulo = PositionA([max[0] as i32, max[1] as i32]);
        Self{grid, position, direction, modulo}
    }

    pub fn move_tiles(&mut self, steps: usize) {
        let movement = PositionA(self.direction.get_move_vector());
        for _ in 0..steps {
            let mut tmp_position = (self.position + movement).rem_euclid(self.modulo);
            while self.floor(&tmp_position) == Floor::Nothing {
                tmp_position = (tmp_position + movement).rem_euclid(self.modulo);
            }
            if self.floor(&tmp_position) == Floor::Wall {
                return;
            } else {
                self.position = tmp_position;
            }
        }
    }

    pub fn change_direction(&mut self, c: char) {
        self.direction = self.direction.change(c);
    }

    pub fn get_position_score(&self) -> usize {
        let row_score = ((self.position.0[0]+1) * 1000) as usize;
        println!("row {}\tscore {}", self.position.0[0], row_score);
        let column_score = ((self.position.0[1]+1) * 4) as usize;
        println!("column {}\tscore {}", self.position.0[1], column_score);
        let direction_score = self.direction.get_index() as usize;
        println!("direction {}\tscore {}", self.direction, direction_score);
        row_score + column_score + direction_score
    }

    fn floor(&self, position: &PositionA) -> Floor {
        assert!(position.0[0] >= 0 && (position.0[0] as usize) < self.grid.len());
        assert!(position.0[1] >= 0 && (position.0[1] as usize) < self.grid[position.0[0] as usize].len());
        self.grid[position.0[0] as usize][position.0[1] as usize]
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Floor {
    Nothing,
    Floor,
    Wall,
}

impl Floor {
    pub fn from(c: char) -> Self {
        match c {
            ' ' => Floor::Nothing,
            '.' => Floor::Floor,
            '#' => Floor::Wall,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn change(self, c: char) -> Self {
        let change = match c {
            'L' => -1,
            'R' => 1,
            _ => panic!(),
        };
        let new_index = (self.get_index()+change).rem_euclid(4);
        let tmp = Self::from_index(new_index);
        tmp
    }

    pub fn get_move_vector(&self) -> [i32; 2] {
        match self {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
        }
    }

    fn get_index(&self) -> i32 {
        match self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }

    fn from_index(index: i32) -> Self {
        assert!(index >= 0);
        assert!(index < 4);
        match index {
            0 => Self::East,
            1 => Self::South,
            2 => Self::West,
            3 => Self::North,
            _ => panic!(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PositionA(
    [i32; 2],
);

impl PositionA {
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let position = [self.0[0].rem_euclid(rhs.0[0]), self.0[1].rem_euclid(rhs.0[1])];
        Self(position)
    }
}

impl Display for PositionA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0[0], self.0[1])
    }
}

impl Add for PositionA {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let position = [self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]];
        Self(position)
    }
}