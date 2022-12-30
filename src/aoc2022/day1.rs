#![allow(dead_code)]

fn main() {
    a();
    b();
}

pub fn a() {
    let input : Vec<String> = advent_of_code_22::read_lines();
    let mut current = 0;
    let mut max = 0;

    for line in input {

        if !line.is_empty() {
            current += line.trim().parse::<i32>().unwrap();
        } else {
            println!("elf parsed: {}", current);
            if current > max {
                max = current;
            }
            current = 0;
        }
    }
    println!("max: {}", max);
}

pub fn b() {
    let input : Vec<String> = advent_of_code_22::read_lines();
    let mut current = 0;
    let mut calories = vec![];

    for line in input {

        if !line.is_empty() {
            current += line.trim().parse::<i32>().unwrap();
        } else {
            println!("elf parsed: {}", current);
            calories.push(current);
            current = 0;
        }
    }
    calories.sort();

    println!("max: {}", calories.pop().unwrap()+calories.pop().unwrap()+calories.pop().unwrap());
}