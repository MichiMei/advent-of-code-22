#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut auto = Automaton::new();
    for line in input {
        parse_command(&line, &mut auto);
    }
    println!("{}", auto.get_sig_strength_sum());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut auto = Automaton::new();
    for line in input {
        parse_command(&line, &mut auto);
    }
}

fn parse_command(command: &str, auto: &mut Automaton) {
    let mut words = command.split(" ");
    let cmd = words.next().unwrap();
    match cmd {
        "noop" => auto.noop(),
        "addx" => {
            let str_param = words.next().unwrap();
            let int_param = str_param.parse::<i32>().unwrap();
            auto.add_x(int_param);
        }
        _ => {}
    }
}

struct Automaton {
    cycle: usize,
    register: i32,
    sig_strength_sum: i32,
}

impl Automaton {
    pub fn new() -> Self {
        Automaton{cycle: 0, register: 1, sig_strength_sum: 0}
    }

    pub fn noop(&mut self) {
        //println!("noop");
        self.advance();
    }

    pub fn add_x(&mut self, x: i32) {
        //println!("addx({})", x);
        self.advance();
        self.advance();
        self.register += x;
    }

    pub fn get_sig_strength_sum(&self) -> i32 {
        self.sig_strength_sum
    }

    fn check_sig_strength(&mut self) {
        if self.cycle%20 == 0 {
            //println!("cycle {} dividable by 20", self.cycle);
            if self.cycle%40 != 0 {
                //println!("cycle {} NOT dividable by 40", self.cycle);
                //println!("cycle {}: {}+{}*{}={}", self.cycle, self.sig_strength_sum, self.cycle, self.register, self.sig_strength_sum+(self.cycle as i32)*self.register);
                self.sig_strength_sum += (self.cycle as i32)*self.register;
            }
        }
    }

    fn advance(&mut self) {
        let normalized_cycle = (self.cycle%40) as i32;

        if normalized_cycle.abs_diff(self.register) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        self.cycle += 1;

        if self.cycle%40 == 0 {
            println!();
        }

        //println!("\tadvanced to {}", self.cycle);
        self.check_sig_strength()
    }
}