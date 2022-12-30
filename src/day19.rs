#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let mut counter = 0;
    for (index, line) in input.iter().enumerate() {
        let res = find_optimum(line, 24);
        println!("{}\t{}", index+1, res);
        counter += (index+1)*(res as usize);
    }
    println!("res: {}", counter);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let mut product = 1;
    for index in 0..3 {
        let res = find_optimum(&input[index], 32);
        println!("{}\t{}", index+1, res);
        product *= res;
    }
    println!("res: {}", product);
}

pub struct Game {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    round: usize,
    cost: Cost,
}

type Resources = [u16; 4];
type Robots = [u16; 4];

pub fn find_optimum(str: &str, rounds: u16) -> u16 {
    let cost = Cost::from(str);
    let mut cache: HashMap<(u16, Resources, Robots), u16> = HashMap::new();

    //println!("{}", cost);
    let mut resources = [0, 0, 0, 0];
    let mut robots = [1, 0, 0, 0];
    find_optimum_rec(&cost, &mut cache, rounds, &mut resources, &mut robots, 0)
}

fn find_optimum_rec(cost: &Cost, cache: &mut HashMap<(u16, Resources, Robots), u16>, rounds_remaining: u16, resources: &Resources, robots: &Robots, goal: u16) -> u16 {
    let mut maximum = max(resources[3], goal);

    if let Some(prev) = cache.get(&(rounds_remaining, *resources, *robots)) {
        //println!("cached");
        return *prev;
    }

    if rounds_remaining == 0 {
        //println!("finished");
        /*println!("resources: {:?}", resources);
        println!("robots: {:?}", robots);
        println!();*/
        return resources[3];
    }

    if !goal_reachable(rounds_remaining, resources, robots, goal) {
        //println!("goal of {} not reachable in {}", goal, rounds_remaining);
        /*println!("resources: {:?}", resources);
        println!("robots: {:?}", robots);
        println!();*/
        return maximum;
    }

    // try to build
    let buildable = cost.buildable(resources, robots);
    for (i, b) in buildable.into_iter().enumerate().rev() {
        if b {
            let mut new_resources = mine_resources(resources, robots);
            let new_robots = cost.build(&mut new_resources, &robots, i);
            let tmp = find_optimum_rec(cost, cache, rounds_remaining-1, &new_resources, &new_robots, maximum);
            maximum = max(maximum, tmp);
        }
    }

    // try no build
    let new_resources = mine_resources(resources, robots);
    let tmp = find_optimum_rec(cost, cache, rounds_remaining-1, &new_resources, robots, maximum);
    maximum = max(maximum, tmp);

    cache.insert((rounds_remaining, resources.clone(), robots.clone()), maximum);
    maximum
}

fn goal_reachable(rounds_remaining: u16, resources: &Resources, robots: &Robots, goal: u16) -> bool {
    let mut max = resources[3] + rounds_remaining*robots[3];
    let sum: u16 = (1..rounds_remaining).sum();
    max += sum;
    if max > goal {
         return true
    }
    false
}

fn mine_resources(resources: &Resources, robots: &Robots) -> Resources {
    [resources[0]+robots[0], resources[1]+robots[1], resources[2]+robots[2], resources[3]+robots[3]]
}

pub struct Cost {
    prices: Vec<Resources>,
    max_prices: Resources,
}

impl Cost {
    pub fn from(str: &str) -> Self {
        let mut prices = vec![];

        let split = str.split(" ");
        let mut split = split.skip(6);
        prices.push([split.next().unwrap().parse::<u16>().unwrap(), 0, 0, 0]);
        let mut split = split.skip(5);
        prices.push([split.next().unwrap().parse::<u16>().unwrap(), 0, 0, 0]);
        let mut split = split.skip(5);
        let obsidian_ore = split.next().unwrap().parse::<u16>().unwrap();
        let mut split = split.skip(2);
        let obsidian_clay = split.next().unwrap().parse::<u16>().unwrap();
        prices.push([obsidian_ore, obsidian_clay, 0, 0]);
        let mut split = split.skip(5);
        let geode_ore = split.next().unwrap().parse::<u16>().unwrap();
        let mut split = split.skip(2);
        let geode_obsidian = split.next().unwrap().parse::<u16>().unwrap();
        prices.push([geode_ore, 0, geode_obsidian, 0]);

        let mut max_prices = [0, 0, 0, 0];
        for price in prices.iter() {
            for index in 0..price.len() {
                max_prices[index] = max(max_prices[index], price[index]);
            }
        }
        max_prices[3] = u16::MAX;

        Cost{prices, max_prices}
    }

    pub fn buildable(&self, resources: &Resources, robots: &Robots) -> Vec<bool> {
        let mut res = vec![];
        for (index, price) in self.prices.iter().enumerate() {
            if price[0] <= resources[0] && price[1] <= resources[1] && price[2] <= resources[2] && price[3] <= resources[3] && robots[index] < self.max_prices[index] {
                res.push(true);
            } else {
                res.push(false);
            }
        }
        res
    }

    pub fn build(&self, resources: &mut Resources, robots: &Robots, index: usize) -> Robots {
        let price = &self.prices[index];
        assert!(resources[0] >= price[0] && resources[1] >= price[1] && resources[2] >= price[2] && resources[3] >= price[3]);

        for index in 0..resources.len() {
            resources[index] -= price[index];
        }

        let mut res = robots.clone();
        res[index] += 1;
        res
    }
}

impl Display for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for price in self.prices.iter() {
            write!(f, "{}\t{}\t{}\t{}\n", price[0], price[1], price[2], price[3]).unwrap();
        }
        write!(f, "")
    }
}