#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let (variables, calculations) = parse_input(&input);
    let (variables, calculations) = execute_solvable(variables, calculations);
    assert!(calculations.is_empty());
    println!("{}", variables.get("root").unwrap());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let (mut variables, mut calculations) = parse_input(&input);

    variables.remove("humn").unwrap();
    let mut root = (String::new(), String::new());
    for index in 0..calculations.len() {
        if calculations[index].variable_name == "root" {
            let removed = calculations.remove(index);
            root.0 = removed.operand0;
            root.1 = removed.operand1;
            break;
        }
    }

    let (variables, calculations) = execute_solvable(variables, calculations);
    println!("solving finished");
    let target;
    if let Some(value) = variables.get(&root.0) {
        target = (root.1, *value);
    } else if let Some(value) = variables.get(&root.1) {
        target = (root.0, *value);
    } else {
        panic!();
    }
    let variables = reverse_execute(variables, calculations, target);

    println!("{}", variables.get("humn").unwrap());
}

pub fn execute_solvable(mut variables: HashMap<String, i64>, mut calculations: Vec<Calculation>) -> (HashMap<String, i64>, Vec<Calculation>) {
    let mut finished = false;
    while !finished {
        finished = true;
        let mut unsolvable = vec![];
        for calculation in calculations {
            if calculation.executable(&variables) {
                let (name, value) = calculation.execute(&variables);
                variables.insert(name, value);
                finished = false;
            } else {
                unsolvable.push(calculation);
            }
        }
        calculations = unsolvable;
    }
    (variables, calculations)
}

pub fn reverse_execute(mut variables: HashMap<String, i64>, mut calculations: Vec<Calculation>, mut target: (String, i64)) -> HashMap<String, i64> {
    while !calculations.is_empty() {
        let mut unsolvable = vec![];
        let mut tmp_target = None;
        let mut changed = false;
        for calculation in calculations {
            if calculation.executable(&variables) {
                let (name, value) = calculation.execute(&variables);
                variables.insert(name, value);
                changed = true;
            } else if calculation.reverse_executable(&variables, &target) {
                let res = calculation.reverse_execute(&variables, &target);
                if tmp_target.is_some() {
                    panic!();
                }
                tmp_target = Some(res);
                changed = true;
            } else {
                unsolvable.push(calculation);
            }
        }
        if !changed {
            panic!()
        }
        calculations = unsolvable;
        if tmp_target.is_some() {
            target = tmp_target.unwrap();
        }
    }
    variables.insert(target.0, target.1);
    variables
}

pub fn parse_input(input: &Vec<String>) -> (HashMap<String, i64>, Vec<Calculation>) {
    let mut variables = HashMap::new();
    let mut calculations = vec![];
    for line in input.iter() {
        if line.len() == 17 {
            calculations.push(Calculation::from(line));
        } else {
            let variable_name = String::from(&line[0..4]);
            let int_str = &line[6..];
            let int = int_str.parse::<i64>().unwrap();
            variables.insert(variable_name, int);
        }
    }
    (variables, calculations)
}

pub struct Calculation {
    variable_name: String,
    operand0: String,
    operand1: String,
    operation: Operation,
}

impl Calculation {
    pub fn from(line: &str) -> Self {
        assert_eq!(line.len(), 17);
        let variable_name = String::from(&line[0..4]);
        let operand0 = String::from(&line[6..10]);
        let operand1 = String::from(&line[13..17]);
        let operation_str = &line[11..12];
        let operation = Operation::from(operation_str);
        Self{variable_name, operand0, operand1, operation}
    }

    pub fn executable(&self, variables: &HashMap<String, i64>) -> bool {
        if !variables.contains_key(&self.operand0) {
            return false
        }
        if !variables.contains_key(&self.operand1) {
            return false
        }
        true
    }

    pub fn reverse_executable(&self, variables: &HashMap<String, i64>, target: &(String, i64)) -> bool {
        /*if self.variable_name == String::from("cczh") && target.0 == String::from("cczh") {
            println!("self {}", self);
            println!("target {:?}", target);
            println!("v0 contained {}", variables.contains_key(&self.operand0));
            println!("v1 contained {}", variables.contains_key(&self.operand1));
            println!("target correct {}", self.operand1 == target.0);
        }*/
        if variables.contains_key(&self.operand0)
            && !variables.contains_key(&self.operand1)
            && self.variable_name == target.0 {
            return true
        }
        if variables.contains_key(&self.operand1)
            && !variables.contains_key(&self.operand0)
            && self.variable_name == target.0 {
            return true
        }
        false
    }

    pub fn execute(self, variables: &HashMap<String, i64>) -> (String, i64) {
        assert!(self.executable(variables));
        let v0 = variables.get(&self.operand0).unwrap();
        let v1 = variables.get(&self.operand1).unwrap();
        let res = self.operation.execute(*v0, *v1);
        (self.variable_name, res)
    }

    pub fn reverse_execute(self, variables: &HashMap<String, i64>, target: &(String, i64)) -> (String, i64) {
        assert!(self.reverse_executable(variables, target));
        if let Some(o0) = variables.get(&self.operand0) {
            let res = self.operation.reverse_execute_operand0(*o0, target.1);
            return (self.operand1, res);
        } else if let Some(o1) = variables.get(&self.operand1) {
            let res = self.operation.reverse_execute_operand1(*o1, target.1);
            return (self.operand0, res);
        } else {
            panic!();
        }
    }
}

impl Display for Calculation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} {} {}", self.variable_name, self.operand0, self.operation, self.operand1)
    }
}

pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    pub fn from(str: &str) -> Self {
        assert_eq!(str.len(), 1);
        return match str.chars().next().unwrap() {
            '+' => Self::Addition,
            '-' => Self::Subtraction,
            '*' => Self::Multiplication,
            '/' => Self::Division,
            _ => panic!(),
        }
    }

    pub fn execute(&self, o0: i64, o1: i64) -> i64 {
        match self {
            Operation::Addition => o0+o1,
            Operation::Subtraction => o0-o1,
            Operation::Multiplication => o0*o1,
            Operation::Division => o0/o1,
        }
    }

    pub fn reverse_execute_operand0(&self, o0: i64, result: i64) -> i64 {
        match self {
            Operation::Addition => result-o0,
            Operation::Subtraction => o0-result,
            Operation::Multiplication => result/o0,
            Operation::Division => o0/result,
        }
    }

    pub fn reverse_execute_operand1(&self, o1: i64, result: i64) -> i64 {
        match self {
            Operation::Addition => result-o1,
            Operation::Subtraction => result+o1,
            Operation::Multiplication => result/o1,
            Operation::Division => result*o1,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Addition => write!(f, "+"),
            Operation::Subtraction => write!(f, "-"),
            Operation::Multiplication => write!(f, "*"),
            Operation::Division => write!(f, "/"),
        }
    }
}