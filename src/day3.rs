#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let mut total_score = 0;

    for line in input {
        let overlap = get_overlap(&line);
        let score = get_score(overlap);
        total_score += score;
    }
    println!("total score {}", total_score);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut total_score = 0;

    let mut iter = input.iter();
    loop {
        let tmp = iter.next();
        if tmp.is_none() {
            break
        }
        let line0 = tmp.unwrap();
        let line1 = iter.next().unwrap();
        let line2 = iter.next().unwrap();


        let overlap = get_overlap_of_three(line0, line1, line2);
        let score = get_score(overlap);
        total_score += score;
    }
    println!("total score {}", total_score);
}

fn get_overlap_of_three(line0: &str, line1: &str, line2: &str) -> u8 {
    assert!(line0.is_ascii());
    assert!(line1.is_ascii());
    assert!(line2.is_ascii());
    let ascii0 = line0.as_bytes();
    let ascii1 = line1.as_bytes();
    let ascii2 = line2.as_bytes();
    for item in ascii0 {
        if ascii1.contains(item) && ascii2.contains(item) {
            return item.clone()
        }
    }
    panic!()
}

fn get_overlap(line: &str) -> u8{
    assert!(line.is_ascii());
    let ascii = line.as_bytes();
    let length = ascii.len();
    assert_eq!(length % 2, 0);
    let half_length = length/2;
    let left = &ascii[..half_length];
    let right = &ascii[half_length..];
    assert_eq!(left.len(), right.len());

    for item in left {
        if right.contains(item) {
            return item.clone();
        }
    }
    panic!()
}

fn get_score(i: u8) -> u32 {
    if i >= ('a' as u8) && i <= ('z' as u8) {
        return (i - ('a' as u8) + 1) as u32
    }
    if i >= ('A' as u8) && i <= ('Z' as u8) {
        return (i - ('A' as u8) + 27) as u32
    }
    panic!()
}