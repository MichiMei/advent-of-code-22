#![allow(dead_code)]

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let elements = parse_input_pairs(&input);

    let mut count = 0;
    for (index, (first, second)) in elements.into_iter().enumerate() {
        assert!(first != second);
        if first < second {
            count += index+1;
            println!("smaller {}", index);
        } else {
            println!("bigger {}", index);
        }
    }
    println!("{}", count);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut elements = parse_input_all(&input);

    let divider_vec = vec![String::from("[[2]]"), String::from("[[6]]")];
    let mut dividers = parse_input_all(&divider_vec);

    elements.append(&mut dividers);

    elements.sort();

    let dividers = parse_input_all(&divider_vec);
    let mut product = 1;
    for divider in dividers {
        let index = elements.iter().position(|x| x==&divider).unwrap();
        product *= index+1;
    }
    println!("{}", product);
}

pub enum Element {
    Int(u32),
    List(Vec<Element>),
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Int(i) => write!(f, "{}", i),
            Element::List(l) => {
                if l.is_empty() {
                    write!(f, "[]")
                } else {
                    let mut iter = l.iter();
                    write!(f, "[{}", iter.next().unwrap()).unwrap();
                    while let Some(val) = iter.next() {
                        write!(f, ",{}", val).unwrap();
                    }
                    write!(f, "]")
                }
            }
        }
    }
}

impl Element {
    pub fn from(str: &str) -> Self {
        let mut chars = str.chars().peekable();
        Element::parse_chars(&mut chars).unwrap()
    }

    fn parse_chars(chars: &mut Peekable<Chars>) -> Option<Element> {
        let c = match chars.next() {
            None => return None,
            Some(c) => c,
        };
        return if c == '[' {
            let mut vec = vec![];
            if *chars.peek().unwrap() == ']' {
                chars.next();
            } else {
                loop {
                    let elem = Element::parse_chars(chars).unwrap();
                    vec.push(elem);
                    match chars.next().unwrap() {
                        ',' => continue,
                        ']' => break,
                        _ => panic!(),
                    }
                }
            }
            Some(Element::List(vec))
        } else {
            let mut str = String::from(c);
            while chars.peek().is_some() {
                let tmp = chars.peek().unwrap();
                if tmp.is_digit(10) {
                    str.push(*tmp);
                    chars.next();
                } else {
                    break;
                }
            }
            let int = str.parse::<u32>().unwrap();
            Some(Element::Int(int))
        }
    }
}

impl PartialEq<Self> for Element {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Element::Int(s) => {
                match other {
                    Element::Int(o) => s == o,
                    Element::List(_) => false,
                }
            }
            Element::List(s) => {
                match other {
                    Element::Int(_) => false,
                    Element::List(o) => s == o,
                }
            }
        }
    }
}

impl Eq for Element {}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self {
            Element::Int(s) => {
                match other {
                    Element::Int(o) => s.cmp(o),
                    Element::List(o) => {
                        let s = vec![Element::Int(*s)];
                        s.cmp(o)
                    }
                }
            }
            Element::List(s) => {
                match other {
                    Element::Int(o) => {
                        let o = vec![Element::Int(*o)];
                        s.cmp(&o)
                    }
                    Element::List(o) => s.cmp(o),
                }
            }
        }
    }
}

pub fn parse_input_pairs(input: &Vec<String>) -> Vec<(Element, Element)> {
    let mut res = vec![];

    let mut iter = input.iter();

    loop {let line0 = match iter.next() {
            None => break,
            Some(str) => str,
        };
        let line1 = iter.next().unwrap();
        iter.next();

        let elem0 = Element::from(line0);
        let elem1 = Element::from(line1);

        res.push((elem0, elem1));
    }

    res
}

pub fn parse_input_all(input: &Vec<String>) -> Vec<Element> {
    let mut res = vec![];

    let mut iter = input.iter();

    loop {let line0 = match iter.next() {
        None => break,
        Some(str) => str,
    };
        let line1 = iter.next().unwrap();
        iter.next();

        let elem0 = Element::from(line0);
        let elem1 = Element::from(line1);

        res.push(elem0);
        res.push(elem1);
    }

    res
}