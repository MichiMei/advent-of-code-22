#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::time::SystemTime;

pub fn a() {
    let input = advent_of_code_22::read_int_list();
    let mut list = WrapAroundList::from(&input);
    list.mix();
    println!("{}", list.calc_coordinates());
}

pub fn b() {
    let now = SystemTime::now();
    let input = advent_of_code_22::read_int_list();
    let mut list = WrapAroundList::from(&input);
    println!("list created {:?}", now.elapsed().unwrap());
    list.multiply_list(811589153);
    println!("list multiplied {:?}", now.elapsed().unwrap());
    for _ in 0..10 {
        list.mix();
    }
    println!("list mixed {:?}", now.elapsed().unwrap());
    println!("{}", list.calc_coordinates());
}

pub struct WrapAroundList {
    list: Vec<(i64, usize)>,

}

impl WrapAroundList {
    pub fn from(vec: &Vec<i32>) -> Self {
        let mut list = vec![];
        for (index, elem) in vec.iter().enumerate() {
            list.push((*elem as i64, index));
        }
        Self{list}
    }

    pub fn mix(&mut self) {
        for index in 0..self.list.len() {
            let position = self.find_index(index);
            let elem = self.list.remove(position);
            let new_position = self.calc_new_pos(position, elem.0);
            self.list.insert(new_position, elem);
            //println!("{}", self)
        }
    }

    pub fn calc_coordinates(&self) -> i64 {
        let position = self.find_element(0).unwrap();

        let pos1 = (position+1000)%self.list.len();
        let pos2 = (position+2000)%self.list.len();
        let pos3 = (position+3000)%self.list.len();

        let val1 = self.list[pos1].0;
        let val2 = self.list[pos2].0;
        let val3 = self.list[pos3].0;

        val1+val2+val3
    }

    pub fn multiply_list(&mut self, factor: i64) {
        for (elem, _) in self.list.iter_mut() {
            *elem *= factor;
        }
    }

    fn find_index(&self, index: usize) -> usize {
        assert!(index < self.list.len());
        for (i, (_, comp)) in self.list.iter().enumerate() {
            if index == *comp {
                return i;
            }
        }
        panic!()
    }

    fn find_element(&self, elem: i64) -> Option<usize> {
        for (i, (comp, _)) in self.list.iter().enumerate() {
            if elem == *comp {
                return Some(i);
            }
        }
        None
    }

    fn calc_new_pos(&self, position: usize, movement: i64) -> usize {
        let mut new_pos = (position as i64) + movement;
        new_pos = new_pos.rem_euclid(self.list.len() as i64);
        return new_pos as usize
    }
}

impl Display for WrapAroundList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (movement, _) in self.list.iter() {
            write!(f, "{}, ", movement).unwrap();
        }
        write!(f, "")
    }
}