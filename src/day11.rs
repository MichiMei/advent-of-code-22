#![allow(dead_code)]

use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut game = parse_input(input, 3, 0);

    for _ in 0..20 {
        game.run();
        game.print();
    }
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut game = parse_input(input, 1, 9699690);

    for i in 0..10000 {
        game.run();
        //game.print();
        if i == 0 {
            game.print()
        }
        if i == 19 {
            game.print()
        }

        if i == 999 {
            game.print()
        }
    }
    game.print();
}

struct Game {
    monkeys: Vec<Monkey>,

}

impl Game {
    pub fn new(monkeys: Vec<Monkey>) -> Game {
        Game{monkeys}
    }

    pub fn run(&mut self) {
        for index in 0..self.monkeys.len() {
            self.monkeys[index].items.reverse();
            //println!("Monkey {}:", index);
            while !self.monkeys[index].is_empty() {
                let (item, target) = self.monkeys[index].handle_item();
                self.monkeys[target].add_item(item);
            }
        }
    }

    pub fn print(&self) {
        for monkey in self.monkeys.iter() {
            println!("{}", monkey);
        }
        println!()
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
    divider: usize,
    modulo: usize,
}

impl Monkey {
    pub fn new(name: String, items: Vec<usize>, operation: Operation, test: usize, true_target: usize, false_target: usize, divider: usize, modulo: usize) -> Self {
        Monkey{name, items, operation, test, true_target, false_target, inspection_count: 0, divider, modulo}
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// returns (item, target)
    pub fn handle_item(&mut self) -> (usize, usize) {
        self.inspection_count += 1;

        let mut item = self.items.pop().unwrap();
        //println!("  Monkey inspects an item with a worry level of {}.", item);
        item = self.operation.run(item);
        //println!("    Worry level {} to {}.", self.operation, item);
        item = item/self.divider.clone();
        //println!("    Monkey gets bored with item. Worry level is divided by {} to {}.", self.divider, item);
        if self.modulo != 0 {
            item = item%self.modulo;
        }

        return if item.clone() % self.test.clone() == 0 {
            //println!("    Current worry level is divisible by {}.", self.test);
            //println!("    Item with worry level {} is thrown to monkey {}.", item, self.true_target);
            (item.clone(), self.true_target)
        } else {
            //println!("    Current worry level is not divisible by {}.", self.test);
            //println!("    Item with worry level {} is thrown to monkey {}.", item, self.false_target);
            (item.clone(), self.false_target)
        }
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: ({}) {:?}", self.name, self.inspection_count, self.items)
    }
}

#[derive(Debug)]
enum Operation {
    Sum(usize),
    Multiplication(usize),
    Square,
}

impl Operation {
    pub fn run(&self, v: usize) -> usize {
        return match self {
            Operation::Sum(x) => x+v,
            Operation::Multiplication(x) => x*v,
            Operation::Square => v.clone()*v,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Sum(x) => write!(f, "increases by {}", x),
            Operation::Multiplication(x) => write!(f, "is multiplied by {}", x),
            Operation::Square => write!(f, "is multiplied by itself"),
        }
    }
}

fn parse_input(input: Vec<String>, divider: usize, modulo: usize) -> Game {
    let mut monkeys = vec![];
    let mut iter = input.iter();

    loop {
        let line = match iter.next() {
            None => break,
            Some(x) => x,
        };
        let name = parse_name(&line);
        let line = iter.next().unwrap();
        let items = parse_items(line);
        let line = iter.next().unwrap();
        let operation = parse_operation(line);
        let line = iter.next().unwrap();
        let test = parse_test(line);
        let line = iter.next().unwrap();
        let true_target = parse_true_target(line);
        let line = iter.next().unwrap();
        let false_target = parse_false_target(line);
        iter.next();

        let monkey = Monkey::new(name, items, operation, test, true_target, false_target, divider, modulo);
        monkeys.push(monkey);
    }

    Game::new(monkeys)
}

fn parse_name(line: &str) -> String {
    let mut words = line.split(" ");
    words.next();
    String::from(words.next().unwrap())
}

fn parse_items(line: &str) -> Vec<usize> {
    let mut words = line.split(": ");
    words.next();
    let items_str = words.next().unwrap();
    let item_split = items_str.split(", ");

    let mut vec = vec![];
    for item in item_split {
        vec.push(item.parse::<usize>().unwrap());
    }

    vec
}

fn parse_operation(line: &str) -> Operation {
    if line.contains(" * ") {
        let mut words = line.split(" * ");
        words.next();
        let str_param = words.next().unwrap();
        if str_param.contains("old") {
            return Operation::Square
        }
        let param = str_param.parse::<usize>().unwrap();
        return Operation::Multiplication(param)
    }
    if line.contains(" + ") {
        let mut words = line.split(" + ");
        words.next();
        let str_param = words.next().unwrap();
        let param = str_param.parse::<usize>().unwrap();
        return Operation::Sum(param)
    }
    panic!()
}

fn parse_test(line: &str) -> usize {
    let str_value = &line[19..];
    str_value.parse::<usize>().unwrap()
}

fn parse_true_target(line: &str) -> usize {
    let str_value = &line[25..];
    str_value.parse::<usize>().unwrap()
}

fn parse_false_target(line: &str) -> usize {
    let str_value = &line[26..];
    str_value.parse::<usize>().unwrap()
}