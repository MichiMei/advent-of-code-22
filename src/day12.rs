#![allow(dead_code)]

use std::cmp::Ordering;

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let (mut grid, start, _) = Grid::from(input);

    let distance = grid.start_calculation(start);

    println!("{}", distance);

    grid.print();
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let (mut grid, _, end) = Grid::from(input);

    let distance = grid.start_calculation_reverse(end);

    println!("{}", distance);

    grid.print();
}

pub struct Grid {
    grid: Vec<Vec<Position>>,
}

impl Grid {
    pub fn from(input: Vec<String>) -> (Self, (usize, usize), (usize, usize)) {
        let mut grid = vec![];
        let mut start = (0,0);
        let mut end = (0,0);
        for (index_h, line) in input.iter().enumerate() {
            let mut row = vec![];
            for (index_w, mut char) in line.chars().enumerate() {
                let mut target = false;
                if char == 'S' {
                    start = (index_h, index_w);
                    char = 'a';
                }
                if char == 'E' {
                    end = (index_h, index_w);
                    target = true;
                    char = 'z';
                }
                let height = (char as u8) - ('a' as u8);
                row.push(Position{height, distance: None, target})
            }
            grid.push(row);
        }

        (Grid{grid}, start, end)
    }

    pub fn start_calculation(&mut self, start: (usize, usize)) -> usize {
        let mut heap = vec![];
        heap.push(HeapElement{point: start, distance: 0});

        while !heap.is_empty() {
            heap.sort();
            let elem = heap.pop().unwrap();
            //println!("handling \t{}\t{}\t{}", elem.point.0, elem.point.1, elem.distance);
            let position = &mut self.grid[elem.point.0][elem.point.1];
            if position.distance.is_some() {
                continue
            }
            position.distance = Some(elem.distance);
            if position.target {
                return position.distance.unwrap()
            }

            self.add_neighbors((elem.point.0, elem.point.1), &mut heap);
        }
        panic!()
    }

    pub fn start_calculation_reverse(&mut self, start: (usize, usize)) -> usize {
        let mut heap = vec![];
        heap.push(HeapElement{point: start, distance: 0});

        while !heap.is_empty() {
            heap.sort();
            let elem = heap.pop().unwrap();
            //println!("handling \t{}\t{}\t{}", elem.point.0, elem.point.1, elem.distance);
            let position = &mut self.grid[elem.point.0][elem.point.1];
            if position.distance.is_some() {
                continue
            }
            position.distance = Some(elem.distance);
            if position.height == 0 {
                return position.distance.unwrap()
            }

            self.add_neighbors_reverse((elem.point.0, elem.point.1), &mut heap);
        }
        panic!()
    }

    fn add_neighbors(&self, position: (usize, usize), heap: &mut Vec<HeapElement>) {
        let own = &self.grid[position.0][position.1];
        if  position.0+1 < self.grid.len() {
            if own.check(&self.grid[position.0+1][position.1]) {
                let elem = HeapElement{point: (position.0+1, position.1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.0 > 0 {
            if own.check(&self.grid[position.0-1][position.1]) {
                let elem = HeapElement{point: (position.0-1, position.1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.1+1 < self.grid[0].len() {
            if own.check(&self.grid[position.0][position.1+1]) {
                let elem = HeapElement{point: (position.0, position.1+1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.1 > 0 {
            if own.check(&self.grid[position.0][position.1-1]) {
                let elem = HeapElement{point: (position.0, position.1-1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
    }

    fn add_neighbors_reverse(&self, position: (usize, usize), heap: &mut Vec<HeapElement>) {
        let own = &self.grid[position.0][position.1];
        if  position.0+1 < self.grid.len() {
            if own.check_reverse(&self.grid[position.0+1][position.1]) {
                let elem = HeapElement{point: (position.0+1, position.1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.0 > 0 {
            if own.check_reverse(&self.grid[position.0-1][position.1]) {
                let elem = HeapElement{point: (position.0-1, position.1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.1+1 < self.grid[0].len() {
            if own.check_reverse(&self.grid[position.0][position.1+1]) {
                let elem = HeapElement{point: (position.0, position.1+1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
        if  position.1 > 0 {
            if own.check_reverse(&self.grid[position.0][position.1-1]) {
                let elem = HeapElement{point: (position.0, position.1-1), distance: own.distance.unwrap()+1};
                heap.push(elem);
            }
        }
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for elem in row.iter() {
                let option = elem.distance;
                if option.is_some() {
                    print!("{}\t", option.unwrap());
                } else {
                    print!("X\t");
                }
            }
            println!()
        }
        println!();
    }
}

pub struct Position {
    height: u8,
    distance: Option<usize>,
    target: bool,
}

impl Position {
    pub fn check(&self, other: &Position) -> bool {
        if other.distance.is_some() {
            return false
        }
        if other.height > self.height+1 {
            return false
        }
        true
    }

    pub fn check_reverse(&self, other: &Position) -> bool {
        if other.distance.is_some() {
            return false
        }
        if other.height < self.height-1 {
            return false
        }
        true
    }
}

struct HeapElement {
    point: (usize, usize),
    distance: usize,
}

impl Eq for HeapElement {}

impl PartialEq<Self> for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        if self.point == other.point &&
            self.distance == other.distance {
            return true
        }
        false
    }
}

impl PartialOrd<Self> for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance.cmp(&other.distance) == Ordering::Equal {
            return self.point.cmp(&other.point)
        } else {
            return self.distance.cmp(&other.distance).reverse()
        }
    }
}