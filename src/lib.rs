use std::fs;
use rand::{Rng};
use rand::prelude::ThreadRng;

#[derive(Clone, PartialEq)]
pub struct Node {
    pub coordinates: (i32, i32),
    items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    profit: i32,
    weight: i32,
}

#[derive(Clone)]
pub struct Chromosome {
    pub path: Vec<Node>,
    pub fitness: f32,
}

impl Chromosome {
    pub fn new(nodes: &Vec<Node>) -> Chromosome {
        let rng = &mut thread_rng();

        let mut path = Vec::new();
        let mut unused_nodes = nodes.clone();
        for i in 0..nodes.len() {
            let index = rng.gen_range(0..unused_nodes.len());
            path.push(unused_nodes[index].clone());
            path[i].items = Vec::new();
            for item in unused_nodes[index].items.clone() {
                if rng.gen_bool(0.2) {
                    path[i].items.push(item.clone());
                }
            }
            unused_nodes.remove(index);
        }

        let fitness = Chromosome::calculate_fitness(&path);

        // create chromosome
        Chromosome {
            path,
            fitness
        }
    }

    fn calculate_fitness(path: &Vec<Node>) -> f32 {
        let renting_rate = 0.1;
        let v_max = 10.0;
        let v_min = 1.0;
        // maximum weight
        let w = 1;
        let nu = (v_max-v_min) / w as f32;

        let mut fitness = 0.0;
        // add profit of all items
        for node in path {
            for item in &node.items {
                fitness += item.profit as f32;
            }
        }

        // subtract renting rate of all items
        let mut weight = 0;
        let mut current_node = &path[0];
        let mut subtrahend = 0.0;
        for node in &path[1..] {
            let current_weight = weight;
            for item in &node.items {
                weight += item.weight;
            }
            if weight > w {
               return 0.0;
            }
            subtrahend += dist(current_node, node) / (v_max - (nu * current_weight as f32));
            current_node = node;
        }
        subtrahend *= renting_rate;
        fitness -= subtrahend;
        fitness
    }

    pub fn crossover(self, other: &Chromosome, nodes: &Vec<Node>) -> Chromosome {
        let n = self.path.len();
        let (start, end) = random_subpath(n);
        let mut child_path = Vec::new();
        for _ in 0..n {
            child_path.push(Node {
                coordinates: (0, 0),
                items: Vec::new(),
            });
        }

        // Copy subpath from parent1 to child
        for i in start..=end {
            child_path[i] = self.path[i].clone();
        }

        // Fill in remaining nodes from parent2
        let mut index = end + 1;
        for i in 0..other.path.len() {
            if !child_path.contains(&other.path[i]) {
                if index == n {
                    index = 0;
                }
                child_path[index] = other.path[i].clone();
                index += 1;
            }
        }

        Chromosome::repair(&mut child_path, nodes);

        let fitness = Chromosome::calculate_fitness(&child_path);

        Chromosome {
            path: child_path,
            fitness,
        }
    }

    pub fn repair(path: &mut Vec<Node>, nodes: &Vec<Node>) {
        let mut unvisited_nodes = Vec::new();
        for node in nodes {
            let mut found = false;
            for solution_node in path.clone() {
                if node.coordinates == solution_node.coordinates {
                    found = true;
                    break;
                }
            }
            if !found {
                unvisited_nodes.push(node.clone());
            }
        }
        // Add missing cities using Nearest Neighbor algorithm
        for i in 0..unvisited_nodes.len() {
            let mut min_dist = f32::MAX;
            let mut min_index = 0;
            for j in 0..path.len() {
                let dist = dist(&unvisited_nodes[i], &path[j]);
                if dist < min_dist {
                    min_dist = dist;
                    min_index = j;
                }
            }
            path.insert(min_index, unvisited_nodes[i].clone());
        }
    }

    pub fn mutate(&mut self, mutation_rate: f32, nodes: &Vec<Node>) {
        let rng = &mut thread_rng();
        for i in 0..self.path.len() {
            if rng.gen_bool(mutation_rate as f64) {
                let index = rng.gen_range(0..self.path.len());
                self.path.swap(i, index);
            }
        }
        Chromosome::repair(&mut self.path, nodes);
        self.fitness = Chromosome::calculate_fitness(&self.path);
    }
}

pub fn get_input_data(file_path: &str) -> Vec<Node> {
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
                coordinates:
                    (line.next().unwrap().parse().unwrap(),
                     line.next().unwrap().parse().unwrap()),
                items: Vec::new(),
            }
        }).collect();

    // get items
    let _ = lines
        .skip_while(|line| !line.starts_with("ITEMS SECTION"))
        .skip(1) // skip ITEMS SECTION line
        .map(|line| {
            let mut line = line.split_whitespace();
            line.next();
            let profit = line.next().unwrap().parse().unwrap();
            let weight = line.next().unwrap().parse().unwrap();
            let node = node_coordinates.get_mut(line.next().unwrap().parse::<usize>().unwrap() - 1).unwrap();
            node.items.push(Item { profit, weight});
        });
    node_coordinates
}

pub fn dist(node1: &Node, node2: &Node) -> f32 {
    let x = (node1.coordinates.0 - node2.coordinates.0).pow(2);
    let y = (node1.coordinates.1 - node2.coordinates.1).pow(2);
    (x as f32 + y as f32).sqrt()
}

pub fn random_subpath(n: usize) -> (usize, usize) {
    let mut rng = thread_rng();
    let start = rng.gen_range(0..n);
    let end = rng.gen_range(0..n);
    if start < end {
        (start, end)
    } else {
        (end, start)
    }
}
