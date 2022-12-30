#![allow(dead_code)]

use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut sensors = vec![];
    let mut max = Point{x: 0, y: 0};
    let mut min = Point{x: 0, y: 0};
    for line in input {
        let sensor = Sensor::from(&line);
        max = max.get_max(&sensor.get_max());
        min = min.get_min(&sensor.get_min());
        sensors.push(sensor);
    }

    println!("min: {}", min);
    println!("max: {}", max);

    //let offset = min.x-1200000;
    let mut ranges = vec![];

    //let y = 10;
    //let y = 2000000;
    let y = 3186981;

    for sensor in sensors.iter() {
        if sensor.row_in_range(y) {
            //println!("{}", sensor);
            let range = sensor.get_blocked_in_row(y);
            println!("{:?}", range);
            ranges.push(range);
        }
    }

    for sensor in sensors.iter() {
        if sensor.location.y == y {
            println!("sensor at {}", sensor.location.x);
        }
        if sensor.beacon.y == y {
            println!("beacon at {}", sensor.beacon.x);
        }
    }
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut sensors = vec![];
    let mut max = Point{x: 0, y: 0};
    let mut min = Point{x: 0, y: 0};
    for line in input {
        let sensor = Sensor::from(&line);
        max = max.get_max(&sensor.get_max());
        min = min.get_min(&sensor.get_min());
        sensors.push(sensor);
    }

    for y in 0..4000000 {
        let mut ranges = vec![];
        for sensor in sensors.iter() {
            if sensor.row_in_range(y) {
                let range = sensor.get_blocked_in_row(y);
                ranges.push(range);
            }
        }
        let (combination, _res) = combine_ranges(&mut ranges);
        if combination.0 > 0 || combination.1 < 4000000 {
            println!("{}: {},{}", y, combination.0, combination.1);
        }
        if y%999999 == 0 {
            println!("{} done", y);
        }
    }
}

pub fn combine_ranges(ranges: &mut Vec<(i32, i32)>) -> ((i32, i32), bool) {
    let mut range = (0,0);
    'main: while !ranges.is_empty() {
        for index in 0..ranges.len() {
            let tmp = ranges[index];
            if (range.0 >= tmp.0 && range.0 <= tmp.1) || (range.1 >= tmp.0 && range.0 <= tmp.1) {
                range.0 = min(tmp.0, range.0);
                range.1 = max(tmp.1, range.1);
                ranges.remove(index);
                continue 'main
            }
        }
        return (range, false)
    }
    (range, true)
}

pub struct Sensor {
    location: Point,
    beacon: Point,
    distance: usize,
}

impl Sensor {
    pub fn from(line: &str) -> Self {
        let mut split = line.split("=");
        split.next();
        let str_x = split.next().unwrap().split(",").next().unwrap();
        let str_y = split.next().unwrap().split(":").next().unwrap();
        let location = Point{x: str_x.parse::<i32>().unwrap(), y: str_y.parse::<i32>().unwrap()};

        let str_x = split.next().unwrap().split(",").next().unwrap();
        let str_y = split.next().unwrap();
        let beacon = Point{x: str_x.parse::<i32>().unwrap(), y: str_y.parse::<i32>().unwrap()};

        let distance = location.distance_manhattan(&beacon);

        Sensor{location, beacon, distance}
    }

    pub fn get_max(&self) -> Point {
        Point{x: max(self.beacon.x, self.location.x), y: max(self.beacon.y, self.location.y)}
    }

    pub fn get_min(&self) -> Point {
        Point{x: min(self.beacon.x, self.location.x), y: min(self.beacon.y, self.location.y)}
    }

    pub fn row_in_range(&self, y: i32) -> bool {
        self.distance >= self.location.dist_to_row(y)
    }

    pub fn get_blocked_in_row(&self, y: i32) -> (i32, i32) {
        let dist = self.location.dist_to_row(y);
        //println!("distance {}", dist);
        let diameter = (self.distance-dist) as i32;
        //println!("diameter {}", diameter);

        (self.location.x-diameter, self.location.x+diameter)
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sensor at {}, beacon at {}, distance {}", self.location, self.beacon, self.distance)
    }
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn get_max(&self, other: &Self) -> Self {
        Point{x: max(self.x, other.x), y: max(self.y, other.y)}
    }

    pub fn get_min(&self, other: &Self) -> Self {
        Point{x: min(self.x, other.x), y: min(self.y, other.y)}
    }

    pub fn distance_manhattan(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }

    pub fn dist_to_row(&self, y: i32) -> usize {
        self.y.abs_diff(y) as usize
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}