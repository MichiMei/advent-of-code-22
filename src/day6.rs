#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let line = input.first().unwrap();

    let (mut buffer, fin) = RingBuffer::new(&line[0..4]);

    let mut counter = 4;

    if fin {
        println!("{}", counter);
        return;
    }

    for char in line[4..].chars() {
        counter += 1;
        if buffer.add(char) {
            println!("{}", counter);
            return;
        }
    }
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let line = input.first().unwrap();

    let (mut buffer, fin) = RingBuffer::new(&line[0..14]);

    let mut counter = 14;

    if fin {
        println!("{}", counter);
        return;
    }

    for char in line[14..].chars() {
        counter += 1;
        if buffer.add(char) {
            println!("{}", counter);
            return;
        }
    }
}

struct RingBuffer {
    size: usize,
    data: Vec<char>,
    next: usize,
}

impl RingBuffer {
    pub fn new(slice: &str) -> (Self, bool) {
        let mut data = vec![];
        let mut finished = true;
        for char in slice.chars() {
            if finished {
                for comp in data.iter() {
                    if comp == &char {
                        finished = false;
                    }
                }
            }
            data.push(char)
        }

        (RingBuffer{size: data.len(), data, next: 0}, finished)
    }

    pub fn add(&mut self, element: char) -> bool {
        self.data[self.next] = element;
        self.next = (self.next+1)%self.size;

        for index0 in 0..self.size {
            for index1 in index0+1..self.size {
                if self.data[index0] == self.data[index1] {
                    return false
                }
            }
        }
        true
    }
}