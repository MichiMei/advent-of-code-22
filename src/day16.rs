#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let system = System::from(input);
    system.print_matrix();
    let res = system.best_path();
    println!("{}", res);
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let system = System::from(input);
    system.print_matrix();
    let res = system.best_double();
    println!("{}", res);
}

pub struct System {
    //valves: HashMap<String, Valve>,
    valves: Vec<Valve>,
    index: HashMap<String, usize>,
    matrix: Vec<Vec<Option<u32>>>,
    apsp: Vec<Vec<u32>>,
}

impl System {
    pub fn from(input: Vec<String>) -> System {
        let mut valves = vec![];

        for line in input {
            let valve = Valve::from(&line);
            valves.push(valve)
        }

        //simplify
        let mut index = 0;
        while index < valves.len() {
            if valves[index].name == "AA" || valves[index].flow_rate > 0 {
                index += 1;
                continue;
            }
            let valve = valves.remove(index);
            for other in valves.iter_mut() {
                other.replace_tunnel(&valve.name, &valve.tunnels);
            }
        }

        // index
        let mut index = HashMap::new();
        for (i, valve) in valves.iter().enumerate() {
            index.insert(valve.name.clone(), i);
        }

        // create matrix
        let mut matrix = vec![];
        matrix.resize(valves.len(), vec![None; valves.len()]);
        for (i, valve) in valves.iter().enumerate() {
            for (name, value) in valve.tunnels.iter() {
                matrix[i][*(index.get(name).unwrap())] = Some(*value);
            }
        }


        // create apsp matrix
        let mut apsp = vec![];
        apsp.resize(valves.len(), vec![u32::MAX; valves.len()]);
        for index in 0..matrix.len() {
            Self::dijkstra(&matrix, &mut apsp, index);
        }

        System{valves, index, matrix, apsp}
    }

    fn dijkstra(matrix: &Vec<Vec<Option<u32>>>, apsp: &mut Vec<Vec<u32>>, node: usize) {
        let mut queue = vec![];
        queue.push((node, 0));
        while !queue.is_empty() {
            queue.sort_by(|x0, x1| {x1.1.cmp(&x0.1)});
            let (current_node, current_dist) = queue.pop().unwrap();
            if apsp[node][current_node] != u32::MAX {
                assert!(apsp[node][current_node] <= current_dist);
                continue;
            }
            apsp[node][current_node] = current_dist;
            for (neighbor_index, neighbor_dist) in matrix[current_node].iter().enumerate() {
                if neighbor_dist.is_some() {
                    queue.push((neighbor_index, current_dist+neighbor_dist.unwrap()));
                }
            }
        }
    }

    pub fn print_matrix(&self) {
        println!("{:?}", self.index);
        Self::print_matrix_raw(&self.apsp);
    }

    fn print_matrix_raw(matrix: &Vec<Vec<u32>>) {
        for line in matrix.iter() {
            for elem in line.iter() {
                print!("{}\t", elem);
            }
            println!();
        }
        println!();
    }

    pub fn best_path(&self) -> u32 {
        let visited = vec![false; self.matrix.len()];
        let path = vec![(String::from("AA"), 1)];
        let (res, _) = self.best_path_rec(*self.index.get("AA").unwrap(), &visited, 30, 0, &path);
        res
    }

    fn best_path_rec(&self, current: usize, visited: &Vec<bool>, rounds_remaining: u32, flow_sum: u32, path: &Vec<(String, u32)>) -> (u32, Vec<(String, u32)>) {
        if rounds_remaining == 0 {
            /*for _ in 0..(30-rounds_remaining) {
                print!(" ");
            }*/
            //println!("finished with {} flow", flow_sum);
            //println!("{}\t{:?}", flow_sum, path);
            return (flow_sum, path.clone());
        }
        let mut finished = true;
        for visited in visited.iter() {
            if !visited {
                finished = false;
                break;
            }
        }
        if finished {
            /*for _ in 0..(30-rounds_remaining) {
                print!(" ");
            }*/
            //println!("finished with {} flow", flow_sum);
            //println!("{}\t{:?}", flow_sum, path);
            return (flow_sum, path.clone());
        }

        let mut maximum = flow_sum;
        let mut best_path = path.clone();

        for (index, v) in visited.iter().enumerate() {
            if *v {
                continue;
            }
            if self.apsp[current][index]+1 > rounds_remaining {
                continue;
            }
            let mut new_visited = visited.clone();
            new_visited[index] = true;
            let new_rounds_remaining = rounds_remaining-self.apsp[current][index]-1;
            let new_flow_sum = flow_sum + new_rounds_remaining*self.valves[index].flow_rate;
            let mut new_path = path.clone();
            new_path.push((self.valves[index].name.clone(), 30-new_rounds_remaining+1));

            /*for _ in 0..(30-rounds_remaining) {
                print!(" ");
            }*/
            //println!("opening valve {} at round {}", self.valves[index].name, 30-new_rounds_remaining+1);
            let (tmp_sum, tmp_path) = self.best_path_rec(index, &new_visited, new_rounds_remaining, new_flow_sum, &new_path);
            if tmp_sum > maximum {
                maximum = tmp_sum;
                best_path = tmp_path;
            }
        }

        (maximum, best_path)
    }

    pub fn best_double(&self) -> u32 {
        let mut visited = vec![false; self.matrix.len()];
        let start = *self.index.get("AA").unwrap();
        let path = vec![(String::from("AA"), 1)];
        visited[start] = true;
        //self.best_double_rec(*self.index.get("AA").unwrap(), *self.index.get("AA").unwrap(), &visited, 26, 26, 0)

        let (first_sum, first_path) = self.best_path_rec(start, &visited, 26, 0, &path);
        println!("first sum: {}", first_sum);

        for (name, _) in first_path {
            visited[*self.index.get(&name).unwrap()] = true;
        }
        let (second_sum, _) = self.best_path_rec(start, &visited, 26, first_sum, &path);
        println!("second sum: {}", second_sum);
        second_sum
    }

    /*fn best_double_rec(&self, c0: usize, c1: usize, visited: &Vec<bool>, rr0: u32, rr1: u32, flow_sum: u32) -> u32 {

    }*/
}

pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: HashMap<String, u32>,
}

impl Valve {
    pub fn from(str: &str) -> Valve {
        let str = str.replace(" valve ", " valves ");
        let name = String::from(&str[6..8]);
        let flow_rate = (&str[23..str.find(";").unwrap()]).parse::<u32>().unwrap();
        let mut split = str.split("valves ");
        split.next();
        let tunnels_split = split.next().unwrap().split(", ");
        let mut tunnels = HashMap::new();
        for tunnel in tunnels_split {
            tunnels.insert(String::from(tunnel), 1);
        }
        Valve{name, flow_rate, tunnels}
    }

    pub fn replace_tunnel(&mut self, target: &str, new: &HashMap<String, u32>) {
        if let Some(current_val) = self.tunnels.remove(target) {
            for (name, val) in new.iter() {
                self.add_tunnel(name, val+current_val);
            }
        }
    }

    fn add_tunnel(&mut self, target: &str, value: u32) {
        if target == self.name {
            return;
        }
        if let Some(current) = self.tunnels.get(target) {
            if value < *current {
                self.tunnels.insert(String::from(target), value);
            }
        } else {
            self.tunnels.insert(String::from(target), value);
        }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Valve {} has flow rate={}; tunnels lead to ", self.name, self.flow_rate).unwrap();
        for tunnel in self.tunnels.iter() {
            write!(f, "{}({}), ", tunnel.0, tunnel.1).unwrap();
        }
        write!(f, "")
    }
}