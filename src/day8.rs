#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let matrix = transform_to_vec(input);

    let count = count_visible(&matrix);
    println!("{}", count);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let matrix = transform_to_vec(input);

    println!("{}", find_best_scenic_score(&matrix));
}

fn transform_to_vec(input: Vec<String>) -> Vec<Vec<u8>> {
    let mut res = vec![];

    for line in input {
        let mut row = vec![];

        for char in line.chars() {
            row.push(char as u8);
        }

        res.push(row);
    }

    res
}

fn count_visible(matrix: &Vec<Vec<u8>>) -> usize {
    let height = matrix.len();
    let width = matrix.first().unwrap().len();

    let mut visible = height*2 + width*2 -4;

    for index_h in 1..(height-1) {
        for index_w in 1..(width-1) {
            if check_visible(matrix, index_h, index_w) {
                visible += 1;
            }
        }
    }

    visible
}

fn check_visible(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> bool {
    if check_row(matrix, index_h, index_w) {
        return true
    }
    if check_col(matrix, index_h, index_w) {
        return true
    }
    return false
}

fn check_row(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> bool {
    let target = matrix[index_h][index_w];
    let mut visible_small = true;
    for i in 0..index_h {
        if matrix[i][index_w] >= target {
            visible_small = false;
        }
    }
    if visible_small {
        return true
    }
    let mut visible_high = true;
    for i in (index_h+1)..matrix.len() {
        if matrix[i][index_w] >= target {
            visible_high = false;
        }
    }
    if visible_high {
        return true
    }
    false
}

fn check_col(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> bool {
    let target = matrix[index_h][index_w];
    let mut visible_small = true;
    for i in 0..index_w {
        if matrix[index_h][i] >= target {
            visible_small = false;
        }
    }
    if visible_small {
        return true
    }
    let mut visible_high = true;
    for i in (index_w+1)..matrix.first().unwrap().len() {
        if matrix[index_h][i] >= target {
            visible_high = false;
        }
    }
    if visible_high {
        return true
    }
    false
}

fn find_best_scenic_score(matrix: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;
    for index_h in 1..(matrix.len()-1) {
        for index_w in 1..(matrix.first().unwrap().len()-1) {
            let res = calculate_scenic_score(matrix, index_h, index_w);
            if res > max {
                max = res;
            }
        }
    }
    max
}

fn calculate_scenic_score(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> usize {
    let mut score = calculate_ss_left(matrix, index_h, index_w);
    score *= calculate_ss_right(matrix, index_h, index_w);
    score *= calculate_ss_top(matrix, index_h, index_w);
    score *= calculate_ss_bot(matrix, index_h, index_w);

    score
}

fn calculate_ss_left(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> usize {
    let target = matrix[index_h][index_w];
    let mut res = 0;
    for i in (0..index_w).rev() {
        res += 1;
        if matrix[index_h][i] >= target {
            break
        }
    }
    res
}

fn calculate_ss_right(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> usize {
    let target = matrix[index_h][index_w];
    let mut res = 0;
    for i in (index_w+1)..matrix.first().unwrap().len() {
        res += 1;
        if matrix[index_h][i] >= target {
            break
        }
    }
    res
}

fn calculate_ss_top(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> usize {
    let target = matrix[index_h][index_w];
    let mut res = 0;
    for i in (0..index_h).rev() {
        res += 1;
        if matrix[i][index_w] >= target {
            break
        }
    }
    res
}

fn calculate_ss_bot(matrix: &Vec<Vec<u8>>, index_h: usize, index_w: usize) -> usize {
    let target = matrix[index_h][index_w];
    let mut res = 0;
    for i in (index_h+1)..matrix.len() {
        res += 1;
        if matrix[i][index_w] >= target {
            break
        }
    }
    res
}