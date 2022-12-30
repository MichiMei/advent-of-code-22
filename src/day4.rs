#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let mut counter = 0;

    for line in input {
        let (range0, range1) = parse_line(&line);
        if contained(&range0, &range1) {
            //println!("is contained");
            counter+=1;
        }  else {
            //println!("is not contained");
        }
    }
    println!("contained counter {}", counter);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut counter = 0;

    for line in input {
        let (range0, range1) = parse_line(&line);
        if overlap(&range0, &range1) {
            println!("has overlap");
            counter+=1;
        }  else {
            println!("has no overlap");
        }
    }
    println!("overlap counter {}", counter);
}

fn parse_line(line: &str) -> ((u32, u32),(u32, u32)) {
    let mut word = line.trim().split(',');
    let next = word.next();
    assert!(next.is_some());
    let first = parse_range(next.unwrap());
    let next = word.next();
    assert!(next.is_some());
    let second = parse_range(next.unwrap());
    (first, second)
}

fn parse_range(word: &str) -> (u32, u32) {
    let mut ints = word.trim().split('-');
    let next = ints.next();
    assert!(next.is_some());
    let first = next.unwrap().parse::<u32>().unwrap();
    let next = ints.next();
    assert!(next.is_some());
    let second = next.unwrap().parse::<u32>().unwrap();
    (first, second)
}

fn contained(range0: &(u32, u32), range1: &(u32, u32)) -> bool {
    if range0.0 <= range1.0 && range0.1 >= range1.1 {
        return true
    }
    if range0.0 >= range1.0 && range0.1 <= range1.1 {
        return true
    }
    return false
}

fn overlap(r0: &(u32, u32), r1: &(u32, u32)) -> bool {
    if contained(&r0, &r1) {
        return true
    }
    let mut range0 = r0;
    let mut range1 = r1;
    if r0.0 > r1.0 {
        range0 = r1;
        range1 = r0;
    }
    if range0.1 >= range1.0 {
        return true
    }
    false
}