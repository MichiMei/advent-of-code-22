use std::io::{BufRead, stdin};

pub fn read_int_list() -> Vec<i32> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        let trimmed = line.trim();
        res.push(match trimmed.parse::<i32>() {
            Ok(int) => int,
            Err(_) => continue
        });
    }
    res
}

pub fn read_lines() -> Vec<String> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        let trimmed = line.trim();
        res.push(String::from(trimmed));
    }
    res
}

pub fn read_lines_untrimmed() -> Vec<String> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        res.push(String::from(line));
    }
    res
}