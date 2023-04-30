use std::fs;
use rand::{Rng, thread_rng};

#[derive(PartialEq)]
pub struct Node {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Item {
    profit: i32,
    weight: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Chromosome {
    pub items: Vec<Item>,
    pub fitness: f32,
}

impl Chromosome {
    pub fn new(items: &Vec<Item>) -> Chromosome {
        let rng = &mut thread_rng();

        let mut items_to_take = Vec::new();
        for item in items.clone() {
            if rng.gen_bool(0.1){
                items_to_take.push(item);
            }
        }
        let fitness = Chromosome::calculate_fitness(&items_to_take);

        // create chromosome
        Chromosome {
            items: items_to_take,
            fitness
        }
    }

    fn calculate_fitness(items: &Vec<Item>) -> f32 {
        // add all nodes of items if they arent there already
        let mut nodes = Vec::new();
        for item in items {
            if !nodes.contains(&Node { x: item.x, y: item.y }) {
                nodes.push(Node { x: item.x, y: item.y });
            }
        }

        let mut fitness = 0.0;
        for node in nodes {
           for item in items {
           }
        }
        todo!()
    }

    pub fn crossover(self, other: &Chromosome) -> Chromosome {
        let mut items = self.items.clone();
        items[..(self.items.len() / 2)]
            .clone_from_slice(&other.items[..(self.items.len() / 2)]);

        let fitness = Chromosome::calculate_fitness(&items);

        Chromosome {
            items,
            fitness
        }
    }

    pub fn mutate(&mut self, mutation_rate: f32) {
        let mut rng = thread_rng();
        for i in 0..self.items.len() {
            if rng.gen_bool(mutation_rate as f64) {
                let index = rng.gen_range(0..self.items.len());
                let temp = self.items[i].clone();
                self.items[i] = self.items[index].clone();
                self.items[index] = temp;
            }
        }
    }
}

pub fn get_input_data(file_path: &str) -> Vec<Item> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    // skip information
    loop {
        if lines.next().unwrap().contains("NODE_COORD_SECTION") {
            break;
        }
    }

    // get node coordinates
    let mut node_coordinates: Vec<Node> = lines
        .clone()
        .take_while(|line| !line.starts_with("ITEMS SECTION"))
        .map(|line| {
            let mut line = line.split_whitespace();
            line.next();
            Node {
                x: line.next().unwrap().parse().unwrap(),
                y: line.next().unwrap().parse().unwrap(),
            }
        }).collect();

    // get items
    lines
        .skip_while(|line| !line.starts_with("ITEMS SECTION"))
        .skip(1) // skip ITEMS SECTION line
        .map(|line| {
            let mut line = line.split_whitespace();
            line.next();
            let profit = line.next().unwrap().parse().unwrap();
            let weight = line.next().unwrap().parse().unwrap();
            let node = node_coordinates.get_mut(line.next().unwrap().parse::<usize>().unwrap() - 1).unwrap();
            Item {
                profit,
                weight,
                x: node.x,
                y: node.y,
            }
        })
        .collect()
}

