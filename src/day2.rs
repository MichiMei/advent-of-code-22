#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let mut score = 0;

    for line in input {
        let (other, own) = parse_line(&line);
        let shape_score = own.get_score();
        let outcome_score = calculate_outcome_score(&other, &own);
        println!("shape_score {}; outcome_score {}; total score {}", shape_score, outcome_score, shape_score+outcome_score);
        score += shape_score;
        score += outcome_score;
    }
    println!("score: {}", score);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let mut score = 0;

    for line in input {
        let (other, outcome) = parse_line2(&line);
        let shape_score = calculate_shape_score(&other, &outcome);
        let outcome_score = outcome.get_score();
        println!("shape_score {}; outcome_score {}; total score {}", shape_score, outcome_score, shape_score+outcome_score);
        score += shape_score;
        score += outcome_score;
    }
    println!("score: {}", score);
}

#[derive(PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

pub enum Outcome {
    Loose,
    Draw,
    Win,
}

impl Shape {
    pub fn parse(c: char) -> Self {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissor,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissor,
            _ => panic!()
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

impl Outcome {
    pub fn parse(c: char) -> Self {
        match c {
            'X' => Outcome::Loose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!()
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Outcome::Loose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn parse_line(line: &str) -> (Shape, Shape) {
    let mut iter = line.chars();
    let other = Shape::parse(iter.next().unwrap());
    iter.next();
    let own = Shape::parse(iter.next().unwrap());
    (other, own)
}

fn parse_line2(line: &str) -> (Shape, Outcome) {
    let mut iter = line.chars();
    let other = Shape::parse(iter.next().unwrap());
    iter.next();
    let outcome = Outcome::parse(iter.next().unwrap());
    (other, outcome)
}

fn calculate_outcome_score(other: &Shape, own: &Shape) -> i32 {
    return if other == own {
        3
    } else if other == &Shape::Rock && own == &Shape::Paper ||
        other == &Shape::Paper && own == &Shape::Scissor ||
        other == &Shape::Scissor && own == &Shape::Rock {
        6
    } else {
        0
    }
}

fn calculate_shape_score(other: &Shape, outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Loose => {
            match other {
                Shape::Rock => Shape::Scissor,
                Shape::Paper => Shape::Rock,
                Shape::Scissor => Shape::Paper,
            }.get_score()
        }
        Outcome::Draw => other.get_score(),
        Outcome::Win => match other {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }.get_score()
    }
}