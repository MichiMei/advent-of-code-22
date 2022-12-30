#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut cache = Cache::from(&input);
    let start = Position{x:1, y:0};
    //let goal = Position{x:6, y:5};
    let goal = Position{x:120, y:26};
    let shortest = breadth_first_search(&mut cache, start, goal, 0, false);
    if shortest.is_none() {
        println!("none found");
    } else {
        println!("{}", shortest.unwrap());
    }
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut cache = Cache::from(&input);

    let start = Position{x:1, y:0};
    //let goal = Position{x:6, y:5};
    let goal = Position{x:120, y:26};

    let first = breadth_first_search(&mut cache, start, goal, 0, false).unwrap();
    println!("goal reached at {}", first);
    let second = breadth_first_search(&mut cache, goal, start, first, false).unwrap();
    println!("start reached at {}", second);
    let third = breadth_first_search(&mut cache, start, goal, second, false).unwrap();
    println!("goal reached again at {}", third);

}

pub fn breadth_first_search(cache: &mut Cache, start: Position, goal: Position, starting_round: usize, verbose: bool) -> Option<usize> {
    let mut deque = VecDeque::new();
    deque.push_back((start, starting_round));
    let mut upper_limit = None;
    let mut path_cache = HashSet::new();

    let mut max_round = 0;

    while !deque.is_empty() {
        let (pos, round) = deque.pop_front().unwrap();
        if upper_limit.is_some() && round >= upper_limit.unwrap() {
            if verbose {
                println!("exceeded");
            }
            continue;
        }
        if path_cache.contains(&(pos, round)) {
            continue;
        }
        path_cache.insert((pos, round));

        if round > max_round {
            max_round = round;
            if verbose {
                println!("new max round is {}", max_round);
            }
        }

        if pos == goal {
            if verbose {
                println!("found solution: {}",round);
            }
            if upper_limit.is_none() {
                upper_limit = Some(round);
                if verbose {
                    println!("found upper limit {}", upper_limit.unwrap());
                }
            } else {
                if upper_limit.is_some() && upper_limit.unwrap() > round {
                    upper_limit = Some(round);
                    if verbose {
                        println!("found better upper limit {}", upper_limit.unwrap());
                    }
                }
            }
        }
        for new_pos in get_viable_moves(cache, pos, round) {
            deque.push_back((new_pos, round+1));
        }
    }

    return upper_limit
}

pub fn get_viable_moves(cache: &mut Cache, pos: Position, round: usize) -> Vec<Position> {
    let max = cache.get_limit();
    let free = cache.create(round+1);
    let mut res = vec![];

    if let Some(north) = Direction::North.advance(pos, max) {
        if free.contains(&north) {
            res.push(north);
        }
    }

    if let Some(east) = Direction::East.advance(pos, max) {
        if free.contains(&east) {
            res.push(east);
        }
    }

    if let Some(south) = Direction::South.advance(pos, max) {
        if free.contains(&south) {
            res.push(south);
        }
    }

    if let Some(west) = Direction::West.advance(pos, max) {
        if free.contains(&west) {
            res.push(west);
        }
    }

    if free.contains(&pos) {
        res.push(pos);
    }

    res
}

pub struct Cache {
    cache: Vec<HashSet<Position>>,
    map: Map,
}

impl Cache {
    pub fn from(input: &Vec<String>) -> Self {
        let mut cache = vec![];
        let map = Map::from(input);
        cache.push(map.get_free());
        Self{cache, map}
    }

    pub fn get_free(&self, round: usize) -> &HashSet<Position> {
        assert!(round < self.cache.len());
        &self.cache[round]
    }

    pub fn create(&mut self, round: usize) -> &HashSet<Position> {
        while round >= self.cache.len() {
            self.create_next();
        }
        return self.get_free(round)
    }

    pub fn get_limit(&self) -> Position {
        Position { x: self.map.grid[0].len(), y: self.map.grid.len() }
    }

    fn create_next(&mut self) {
        self.map.advance();
        self.cache.push(self.map.get_free());
    }
}

pub struct Map {
    grid: Vec<Vec<Floor>>,
    max: Position,
}

impl Map {
    pub fn from(input: &Vec<String>) -> Self {
        let mut grid = vec![];
        grid.reserve(input.len());
        let col_size = input[0].len();
        for line in input.iter() {
            let mut row = vec![];
            row.reserve(col_size);
            for char in line.chars() {
                row.push(Floor::from(char));
            }
            grid.push(row);
        }
        let max = Position{x: col_size-1, y: grid.len()-1};
        Self{grid, max}
    }

    pub fn advance(&mut self) {
        let mut new_grid = vec![vec![Floor::Floor(vec![]); self.grid[0].len()]; self.grid.len()];
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, elem) in row.iter().enumerate() {
                match elem {
                    Floor::Wall => new_grid[row_index][col_index] = Floor::Wall,
                    Floor::Floor(blizzards) => {
                        let pos = Position{ x: col_index, y: row_index };
                        for blizzard in blizzards {
                            let new_pos = blizzard.advance(pos, self.max);
                            new_grid[new_pos.y][new_pos.x].add_blizzard(*blizzard);
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    pub fn get_free(&self) -> HashSet<Position> {
        let mut res = HashSet::new();
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, elem) in row.iter().enumerate() {
                match elem {
                    Floor::Wall => {}
                    Floor::Floor(blizzards) => {
                        if blizzards.is_empty() {
                            res.insert(Position{ x: col_index, y: row_index });
                        }
                    }
                }
            }
        }
        res
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for elem in row.iter() {
                write!(f, "{}", elem).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

#[derive(Clone, Debug)]
pub enum Floor {
    Wall,
    Floor(Vec<Blizzard>),
}

impl Floor {
    pub fn from(c: char) -> Floor {
        match c {
            '#' => Floor::Wall,
            '.' => Floor::Floor(vec![]),
            '^' => Floor::Floor(vec![Blizzard{dir: Direction::North}]),
            '>' => Floor::Floor(vec![Blizzard{dir: Direction::East}]),
            'v' => Floor::Floor(vec![Blizzard{dir: Direction::South}]),
            '<' => Floor::Floor(vec![Blizzard{dir: Direction::West}]),
            _ => panic!(),
        }
    }

    pub fn add_blizzard(&mut self, blizzard: Blizzard) {
        match self {
            Floor::Wall => panic!(),
            Floor::Floor(blizzards) => blizzards.push(blizzard),
        }
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Floor::Wall => write!(f, "#"),
            Floor::Floor(vec) => {
                if vec.is_empty() {
                    write!(f, ".")
                } else if vec.len() == 1 {
                    write!(f, "{}", vec[0])
                } else {
                    write!(f, "{}", vec.len())
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Blizzard {
    dir: Direction,
}

impl Blizzard {
    pub fn advance(&self, pos: Position, max: Position) -> Position {
        self.dir.advance_blizzard(pos, max)
    }
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dir)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn advance_blizzard(&self, pos: Position, max: Position) -> Position {
        match self {
            Direction::North => {
                if pos.y - 1 == 0 {
                    Position { x: pos.x, y: max.y - 1 }
                } else {
                    Position { x: pos.x, y: pos.y - 1 }
                }
            }
            Direction::East => {
                if pos.x + 1 == max.x {
                    Position { x: 0 + 1, y: pos.y }
                } else {
                    Position { x: pos.x + 1, y: pos.y }
                }
            }
            Direction::South => {
                if pos.y + 1 == max.y {
                    Position { x: pos.x, y: 0 + 1 }
                } else {
                    Position { x: pos.x, y: pos.y + 1 }
                }
            }
            Direction::West => {
                if pos.x - 1 == 0 {
                    Position { x: max.x - 1, y: pos.y }
                } else {
                    Position { x: pos.x - 1, y: pos.y }
                }
            }
        }
    }

    pub fn advance(&self, pos: Position, max: Position) -> Option<Position> {
        match self {
            Direction::North => {
                if pos.y == 0 {
                    None
                } else {
                    Some(Position { x: pos.x, y: pos.y - 1 })
                }
            }
            Direction::East => {
                if pos.x + 1 == max.x {
                    None
                } else {
                    Some(Position { x: pos.x + 1, y: pos.y })
                }
            }
            Direction::South => {
                if pos.y + 1 == max.y {
                    None
                } else {
                    Some(Position { x: pos.x, y: pos.y + 1 })
                }
            }
            Direction::West => {
                if pos.x == 0 {
                    None
                } else {
                    Some(Position { x: pos.x - 1, y: pos.y })
                }
            }
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