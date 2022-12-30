#![allow(dead_code)]

use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub fn a() {
    let input = advent_of_code_22::read_lines();

    let data_tree = DataTree::new(input);

    println!("{}", data_tree.account_smaller_dirs(100000));
}

pub fn b() {
    let input = advent_of_code_22::read_lines();

    let data_tree = DataTree::new(input);

    let max_size = 70000000;
    let necessary = 30000000;
    let filled = data_tree.get_total_size();
    let minimum_free_up = filled-(max_size-necessary);

    println!("to free up {}", minimum_free_up);
    println!("{:?}", data_tree.get_smallest_bigger(minimum_free_up));
}

struct DataTree {
    root_directory: Directory,
}

impl DataTree {
    pub fn new(commands: Vec<String>) -> Self {
        let mut root = Directory::new(String::from("/"));

        let mut iter = commands.iter().peekable();
        iter.next();

        root.build_from_commands(&mut iter);

        root.calc_sizes();

        DataTree{root_directory: root}
    }

    pub fn calc_sizes(&mut self) {
        self.root_directory.calc_sizes();
    }

    pub fn account_smaller_dirs(&self, comp: usize) -> usize {
        self.root_directory.account_smaller_dirs(comp)
    }

    pub fn get_total_size(&self) -> usize{
        self.root_directory.size
    }

    pub fn get_smallest_bigger(&self, comp: usize) -> Option<usize> {
        self.root_directory.get_smallest_bigger(comp)
    }
}

struct Directory {
    name: String,
    sub_directories: HashMap<String, Directory>,
    files: HashMap<String, File>,
    size: usize,
}

impl Directory {
    pub fn new(name: String) -> Self {
        let sub_directories: HashMap<String, Directory> = HashMap::new();
        let files: HashMap<String, File> = HashMap::new();

        Directory{name, sub_directories, files, size: 0}
    }

    ///
    /// Returns true if back to parent
    /// Returns false if back to root
    ///
    pub fn build_from_commands(&mut self, commands: &mut Peekable<Iter<String>>) -> ReturnStatus {

        let mut next = commands.next();
        while next.is_some() {
            let command = next.unwrap();

            assert!(is_command(&command));

            println!("-build- command: {}", command);

            // parse command
            if command.starts_with("$ cd") {
                match self.cd(command, commands) {
                    ReturnStatus::Finished => {}
                    ReturnStatus::Root => return ReturnStatus::Root,
                    ReturnStatus::Exit => return ReturnStatus::Exit,
                }
            } else if command.starts_with("$ ls") {
                self.ls(commands);
            } else {
                assert!(true);
            }

            // get next command
            next = commands.next();
        }

        return ReturnStatus::Root
    }

    fn cd(&mut self, command: &str, commands: &mut Peekable<Iter<String>>) -> ReturnStatus {
        return if command.contains("/") {
            ReturnStatus::Root
        } else if command.contains("..") {
            ReturnStatus::Exit
        } else {
            let mut words = command.split(" ");
            words.next();
            words.next();
            let directory = words.next().unwrap();

            let sub_dir = self.sub_directories.get_mut(directory).unwrap();
            return match sub_dir.build_from_commands(commands) {
                ReturnStatus::Finished => ReturnStatus::Finished,
                ReturnStatus::Root => ReturnStatus::Root,
                ReturnStatus::Exit => ReturnStatus::Finished,
            }
        }
    }

    fn ls(&mut self, commands: &mut Peekable<Iter<String>>) {
        let mut peek = commands.peek().unwrap();
        while !peek.starts_with("$") {
            let next = commands.next().unwrap();

            let mut words = next.split(" ");
            let size = words.next().unwrap();
            let name = words.next().unwrap();

            if size.contains("dir") {
                self.add_subdirectory(String::from(name));
            } else {
                let int_size = size.parse::<usize>().unwrap();
                self.add_file(String::from(name), int_size);
            }

            let option = commands.peek();
            if option.is_none() {
                return;
            }
            peek = option.unwrap();
        }
    }

    fn add_subdirectory(&mut self, name: String) {
        println!("adding directory {}", name);
        let dir = Directory::new(name.clone());
        if self.sub_directories.insert(name, dir).is_some() {
            assert!(true)
        }
    }

    fn add_file(&mut self, name: String, size: usize) {
        println!("adding file {} {}", name, size);
        let file = File::new(name.clone(), size);
        if self.files.insert(name, file).is_some() {
            assert!(true)
        }
    }

    pub fn calc_sizes(&mut self) -> usize {
        let mut size = 0;
        for (_, file) in self.files.iter() {
            size += file.size;
        }
        for (_, dir) in self.sub_directories.iter_mut() {
            size += dir.calc_sizes();
        }
        self.size = size;
        size
    }

    pub fn account_smaller_dirs(&self, comp: usize) -> usize {
        let mut sum = 0;
        for (_, dir) in self.sub_directories.iter() {
            sum += dir.account_smaller_dirs(comp);
        }
        if self.size <= comp {
            sum += self.size;
        }
        sum
    }

    pub fn get_smallest_bigger(&self, comp: usize) -> Option<usize> {
        let mut min = None;
        for (_, dir) in self.sub_directories.iter() {
            let res = dir.get_smallest_bigger(comp);
            if res.is_some() {
                if min.is_some() {
                    if res.unwrap() < min.unwrap() {
                        min = res;
                    }
                } else {
                    min = res;
                }
            }
        }

        if min.is_none() && self.size > comp {
            min = Some(self.size);
        }

        min
    }
}

fn is_command(command: &str) -> bool {
    if !command.starts_with("$") {
        return false
    }

    return true
}

struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File{name, size}
    }
}

enum ReturnStatus {
    Finished,
    Root,
    Exit
}