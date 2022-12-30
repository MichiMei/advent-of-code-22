#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines_untrimmed();

    let mut index = 0;

    let size = input[0].len();

    let mut crate_place = CratePlace::new((size+3)/4);

    while input[index].contains('[') {
        crate_place.add_row(&input[index]);
        index += 1;
    }

    index += 2;

    while index < input.len() {
        crate_place.execute_command_a(&input[index]);
        index += 1;
    }

    crate_place.print_top_row()
}

pub fn b() {
    let input = advent_of_code_22::read_lines_untrimmed();

    let mut index = 0;

    let size = input[0].len();

    let mut crate_place = CratePlace::new((size+3)/4);

    while input[index].contains('[') {
        crate_place.add_row(&input[index]);
        index += 1;
    }

    index += 2;

    while index < input.len() {
        crate_place.execute_command_b(&input[index]);
        index += 1;
    }

    crate_place.print_top_row()
}

struct CratePlace {
    stacks: Vec<Vec<char>>,
    reversed: bool,
}

impl CratePlace {
    pub fn new(size: usize) -> Self {
        println!("new({})", size);
        CratePlace{stacks: vec![vec![]; size], reversed: false}
    }

    pub fn add_row(&mut self, str: &str) {
        assert!(!self.reversed);

        let mut iter = str.chars();

        for index in 0..self.stacks.len() {
            iter.next().expect("Malformed input (0)");
            let item = iter.next().expect("Malformed input (1)");
            iter.next().expect("Malformed input (2)");
            iter.next();

            if item.is_alphabetic() {
                self.stacks[index].push(item);
            }
        }
    }

    pub fn execute_command_a(&mut self, command: &str) {
        if !self.reversed {
            self.reverse();
        }

        let (src, dest, amount) = parse_command(&command);

        for _ in 0..amount {
            let item = self.stacks[src].pop().expect("Command not executable");
            self.stacks[dest].push(item);
        }
    }

    pub fn execute_command_b(&mut self, command: &str) {
        if !self.reversed {
            self.reverse();
        }

        let (src, dest, amount) = parse_command(&command);

        let mut tmp = vec![];

        for _ in 0..amount {
            let item = self.stacks[src].pop().expect("Command not executable");
            tmp.push(item);
        }

        for _ in 0..amount {
            let item = tmp.pop().unwrap();
            self.stacks[dest].push(item);
        }
    }

    pub fn print_top_row(&self) {
        assert!(self.reversed);

        for stack in self.stacks.iter() {
            print!("{}", stack.last().expect("Stack empty"));
        }
        println!();
    }

    fn reverse(&mut self) {
        self.reversed = true;
        for stack in self.stacks.iter_mut() {
            stack.reverse();
        }
    }
}

fn parse_command(command: &str) -> (usize, usize, usize) {
    let mut words = command.split(' ');

    words.next().expect("Command malformed (0)");
    let amount_str = words.next().expect("Command malformed (1)");
    words.next().expect("Command malformed (2)");
    let src_str = words.next().expect("Command malformed (3)");
    words.next().expect("Command malformed (4)");
    let dest_str = words.next().expect("Command malformed (5)");

    let amount = amount_str.parse::<usize>().expect("Command malformed (6)");
    let src = src_str.parse::<usize>().expect("Command malformed (7)");
    let dest = dest_str.parse::<usize>().expect("Command malformed (8)");

    (src-1, dest-1, amount)
}