#![allow(dead_code)]

use std::fmt::{Display, Formatter};
//use std::ops::Add;

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut sum = 0;
    for line in input {
        let snafu = SNAFU::from(&line);
        let int = snafu.to_int();
        sum += int;
    }
    let snafu = SNAFU::from_int(sum);
    println!("{}", snafu);
}

pub fn b() {

}

#[derive(Default)]
pub struct SNAFU {
    digits: [SNAFUDigit; 32],
}

impl SNAFU {
    pub fn from(str: &str) -> Self {
        let mut snafu = SNAFU::default();
        let length = str.len();
        let mut chars = str.chars();
        for index in snafu.digits.len()-length..snafu.digits.len() {
            snafu.digits[index] = SNAFUDigit::from(chars.next().unwrap());
        }
        assert!(chars.next().is_none());
        snafu
    }

    pub fn to_int(&self) -> u64 {
        let mut sum = 0;
        for digit in self.digits.iter() {
            sum = sum*5 + digit.get_value();
        }
        assert!(sum >= 0);
        sum as u64
    }

    pub fn from_int(mut int: u64) -> Self {
        let mut snafu = SNAFU::default();
        let mut index = snafu.digits.len()-1;
        while int > 0 {
            let rem = int%5;
            let (digit, carry) = SNAFUDigit::from_int(rem);
            snafu.digits[index] = digit;
            int = (int+carry)/5;
            index -= 1;
        }
        snafu
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut leading_zeros = true;
        for digit in self.digits.iter() {
            if leading_zeros && *digit == SNAFUDigit::Zero {
                continue;
            } else {
                write!(f, "{}", digit).unwrap();
                leading_zeros = false
            }
        }
        if leading_zeros {
            write!(f, "{}", SNAFUDigit::Zero).unwrap();
        }
        write!(f, "")
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SNAFUDigit {
    Minus2,
    Minus1,
    Zero,
    Plus1,
    Plus2,
}

impl SNAFUDigit {
    pub fn from(char: char) -> Self {
        match char {
            '=' => SNAFUDigit::Minus2,
            '-' => SNAFUDigit::Minus1,
            '0' => SNAFUDigit::Zero,
            '1' => SNAFUDigit::Plus1,
            '2' => SNAFUDigit::Plus2,
            _ => panic!(),
        }
    }

    pub fn get_value(&self) -> i64 {
        match self {
            SNAFUDigit::Minus2 => -2,
            SNAFUDigit::Minus1 => -1,
            SNAFUDigit::Zero => 0,
            SNAFUDigit::Plus1 => 1,
            SNAFUDigit::Plus2 => 2,
        }
    }

    pub fn from_int(int: u64) -> (Self, u64) {
        match int {
            0 => (Self::Zero, 0),
            1 => (Self::Plus1, 0),
            2 => (Self::Plus2, 0),
            3 => (Self::Minus2, 2),
            4 => (Self::Minus1, 1),
            _ => panic!(),
        }
    }
}

impl Default for SNAFUDigit {
    fn default() -> Self {
        Self::Zero
    }
}

impl Display for SNAFUDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SNAFUDigit::Minus2 => write!(f, "="),
            SNAFUDigit::Minus1 => write!(f, "-"),
            SNAFUDigit::Zero => write!(f, "0"),
            SNAFUDigit::Plus1 => write!(f, "1"),
            SNAFUDigit::Plus2 => write!(f, "2"),
        }
    }
}