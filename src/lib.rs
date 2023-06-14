use std::fs;
use rand::{Rng};
use rand::prelude::ThreadRng;

pub struct Settings {
    pub number_of_generations: i32,
    pub population_size: usize,
    pub item_chance: f64,
    pub mutation_rate: f32,
    pub tournament_size: i32,
    pub elitism: bool,
    pub elitism_size: usize,
    pub renting_rate: f32,
    pub v_max: f32,
    pub v_min: f32,
    pub maximum_weight: i32,
    pub data_path: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Node {
    id: i32,
    pub coordinates: (i32, i32),
    items: Vec<Item>,
}

#[derive(Clone, PartialEq)]
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
    /// create new chromosome with random path
    pub fn new(nodes: &Vec<Node>, rng: &mut ThreadRng, settings: &Settings) -> Chromosome {
        let mut nodes = nodes.clone();
        let Settings { item_chance, .. } = settings;

        // get path with random nodes with random number of items
        let path: Vec<Node> = (0..nodes.len())
            .map(|_| {
                let node = nodes.swap_remove(rng.gen_range(0..nodes.len()));
                let items = node.items
                    .into_iter()
                    .filter(|_| rng.gen_bool(*item_chance))
                    .collect();
                Node {
                    id: node.id,
                    coordinates: node.coordinates,
                    items,
                }
            })
            .collect();

        // get path fitness
        let fitness = Chromosome::calculate_fitness(&path, settings);

        // create chromosome
        Chromosome {
            path,
            fitness,
        }
    }

    /// calculate fitness of path
    fn calculate_fitness(path: &Vec<Node>, settings: &Settings) -> f32 {
        /*
        fitness = sum of profits of all items - (sum of distances between nodes / (v_max - (nu * current_weight)))
        */
        let Settings { renting_rate, v_max, v_min, maximum_weight, .. } = settings;
        let nu = (v_max-v_min) / *maximum_weight as f32;

        // initialize fitness
        let mut fitness = 0.0;

        // add profit of all items
        for node in path {
            for item in &node.items {
                fitness += item.profit as f32;
            }
        }

        // while going through path,
        // thief travels distance, more distance = less fitness
        // thief carries more weight, more weight = less fitness
        let mut current_weight = 0;
        let mut current_node = &path[0];
        let mut subtrahend = 0.0;
        for node in path {
            // add weight of items in current node to current weight
            for item in &node.items {
                current_weight += item.weight;
            }

            // if current weight is greater than what thief can hold at once, return 0
            if current_weight > *maximum_weight {
               return 0 as f32;
            }

            // calculate subtrahend
            subtrahend += dist(node, current_node) as f32 / (v_max - (nu * current_weight as f32));
            current_node = node;
        }

        // multiply subtrahend by renting rate
        subtrahend *= renting_rate;

        // subtract subtrahend from fitness
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
