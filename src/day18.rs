#![allow(dead_code)]

pub fn a() {
    let input = advent_of_code_22::read_lines();
    let matrix = Matrix::from(&input);
    println!("{}", matrix.calc_surface_all_surface());
}

pub fn b() {
    let input = advent_of_code_22::read_lines();
    let matrix = Matrix::from(&input);

    //let (x, y, z) = matrix.count();
    //println!("lava {}, outside {}, inside {}", x, y, z);

    println!("{}", matrix.calc_surface_outside());
}

pub struct Matrix {
    matrix: Vec<Vec<Vec<Element>>>,
}

impl Matrix {
    pub fn from(input: &Vec<String>) -> Self {
        let mut tmp = vec![];
        let mut max = (0, 0, 0);
        for line in input.iter() {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            let z = split.next().unwrap().parse::<usize>().unwrap();
            if x > max.0 {
                max.0 = x;
            }
            if y > max.1 {
                max.1 = y;
            }
            if z > max.2 {
                max.2 = z;
            }
            tmp.push((x,y,z));
        }
        let mut matrix = vec![];
        matrix.resize(max.0+1, vec![]);
        for row in matrix.iter_mut() {
            row.resize(max.1+1, vec![Element::Inside; max.2+1]);
        }

        for (x, y, z) in tmp {
            matrix[x][y][z] = Element::Lava;
        }

        println!("2,2,5 is {:?}", matrix[2][2][5]);

        let mut matrix = Matrix{matrix};
        matrix.find_outside();

        println!("2,2,5 is {:?}", matrix.matrix[2][2][5]);

        matrix
    }

    pub fn calc_surface_all_surface(&self) -> usize {
        let mut surface = 0;
        for (x, x_row) in self.matrix.iter().enumerate() {
            for (y, y_row) in x_row.iter().enumerate() {
                for (z, z_row) in y_row.iter().enumerate() {
                    if *z_row == Element::Lava {
                        surface += self.calc_surface_cube_all_surface(x, y, z);
                    }
                }
            }
        }
        surface
    }

    pub fn calc_surface_outside(&self) -> usize {
        let mut surface = 0;
        for (x, x_row) in self.matrix.iter().enumerate() {
            for (y, y_row) in x_row.iter().enumerate() {
                for (z, z_row) in y_row.iter().enumerate() {
                    if *z_row == Element::Lava {
                        surface += self.calc_surface_cube_outside(x, y, z);
                    }
                }
            }
        }
        surface
    }

    pub fn count(&self) -> (usize, usize, usize) {
        let mut lava = 0;
        let mut outside = 0;
        let mut inside = 0;
        for x_row in self.matrix.iter() {
            for y_row in x_row.iter() {
                for z_row in y_row.iter() {
                    match z_row {
                        Element::Lava => lava += 1,
                        Element::Outside => outside += 1,
                        Element::Inside => inside += 1,
                    }
                }
            }
        }
        (lava, outside, inside)
    }

    fn calc_surface_cube_all_surface(&self, x: usize, y: usize, z: usize) -> usize {
        assert!(self.matrix[x][y][z] == Element::Lava);

        let mut surface = 6;
        if x > 0 && !self.matrix[x-1][y][z].is_air() {
            surface -= 1;
        }
        if x+1 < self.matrix.len() && !self.matrix[x+1][y][z].is_air() {
            surface -= 1;
        }
        if y > 0 && !self.matrix[x][y-1][z].is_air() {
            surface -= 1;
        }
        if y+1 < self.matrix[x].len() && !self.matrix[x][y+1][z].is_air() {
            surface -= 1;
        }
        if z > 0 && !self.matrix[x][y][z-1].is_air() {
            surface -= 1;
        }
        if z+1 < self.matrix[x][y].len() && !self.matrix[x][y][z+1].is_air() {
            surface -= 1;
        }

        surface
    }

    fn calc_surface_cube_outside(&self, x: usize, y: usize, z: usize) -> usize {
        assert!(self.matrix[x][y][z] == Element::Lava);

        let mut surface = 6;
        if x > 0 && !self.matrix[x-1][y][z].is_outside() {
            surface -= 1;
        }
        if x+1 < self.matrix.len() && !self.matrix[x+1][y][z].is_outside() {
            surface -= 1;
        }
        if y > 0 && !self.matrix[x][y-1][z].is_outside() {
            surface -= 1;
        }
        if y+1 < self.matrix[x].len() && !self.matrix[x][y+1][z].is_outside() {
            surface -= 1;
        }
        if z > 0 && !self.matrix[x][y][z-1].is_outside() {
            surface -= 1;
        }
        if z+1 < self.matrix[x][y].len() && !self.matrix[x][y][z+1].is_outside() {
            surface -= 1;
        }

        surface
    }

    fn find_outside(&mut self) {
        for x in 0..self.matrix.len() {
            for y in 0..self.matrix[x].len() {
                for z in 0..self.matrix[x][y].len() {
                    if x > 0 && y > 0 && z > 0 && x < self.matrix.len()-1 && y < self.matrix[x].len()-1 && z < self.matrix[x][y].len()-1 {
                        continue
                    }
                    if self.matrix[x][y][z] == Element::Inside {
                        self.mark_outside(x, y, z);
                    }
                }
            }
        }
    }

    fn mark_outside(&mut self, x: usize, y: usize, z: usize) {
        let mut next = vec![];
        next.push((x, y, z));
        while !next.is_empty() {
            let (x, y, z) = next.pop().unwrap();
            if self.matrix[x][y][z] == Element::Inside {
                self.matrix[x][y][z] = Element::Outside;
                self.push_neighbors(x, y, z, &mut next);
            }
        }

    }

    fn push_neighbors(&self, x: usize, y: usize, z: usize, next: &mut Vec<(usize, usize, usize)>) {
        if x > 0 && self.matrix[x-1][y][z] == Element::Inside {
            next.push((x-1, y, z));
        }
        if x+1 < self.matrix.len() && self.matrix[x+1][y][z] == Element::Inside {
            next.push((x+1, y, z));
        }
        if y > 0 && self.matrix[x][y-1][z] == Element::Inside {
            next.push((x, y-1, z));
        }
        if y+1 < self.matrix[x].len() && self.matrix[x][y+1][z] == Element::Inside {
            next.push((x, y+1, z));
        }
        if z > 0 && self.matrix[x][y][z-1] == Element::Inside {
            next.push((x, y, z-1));
        }
        if z+1 < self.matrix[x][y].len() && self.matrix[x][y][z+1] == Element::Inside {
            next.push((x, y, z+1));
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Element {
    Lava,
    Outside,
    Inside,
}

impl Element {
    pub fn is_outside(&self) -> bool {
        if *self == Element::Outside {
            return true
        }
        false
    }

    pub fn is_air(&self) -> bool {
        if *self == Element::Lava {
            return false
        }
        true
    }
}