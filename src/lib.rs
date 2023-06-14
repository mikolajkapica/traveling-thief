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

    /// make child from 2 parents by combining parts of both parents
    pub fn crossover(self, other: &Chromosome, nodes: &Vec<Node>, rng: &mut ThreadRng, settings: &Settings) -> Chromosome {
        let path_length = self.path.len();
        let (start, end) = random_subpart(path_length, rng);

        // create child path
        let mut child_path = Vec::with_capacity(path_length);
        for _ in 0..path_length {
            child_path.push(Node {
                id: 0,
                coordinates: (0, 0),
                items: Vec::new(),
            });
        }

        // fill in nodes from parent1
        for i in start..=end { child_path[i] = self.path[i].clone(); }

        // fill in nodes from parent2
        for i in 0..start { child_path[i] = other.path[i].clone(); }
        for i in end+1..path_length { child_path[i] = other.path[i].clone(); }

        // repair child path
        Chromosome::repair(&mut child_path, &nodes);

        // calculate fitness
        let fitness = Chromosome::calculate_fitness( &child_path, settings);

        // return child
        Chromosome {
            path: child_path,
            fitness,
        }
    }

    /// swap 2 random nodes in path
    pub fn mutate(&mut self, rng: &mut ThreadRng, settings: &Settings) {
        let Settings { mutation_rate, .. } = settings;

        // swap 2 random nodes in path
        for i in 0..self.path.len() {
            if rng.gen_bool(*mutation_rate as f64) {
                let index = rng.gen_range(0..self.path.len());
                self.path.swap(i, index);
            }
        }

        // recalculate fitness
        self.fitness = Chromosome::calculate_fitness(&self.path, settings);
    }

    /// repair path by adding missing cities using Nearest Neighbor algorithm
    fn repair(path: &mut Vec<Node>, nodes: &Vec<Node>) {
        // if we have duplicate nodes in path, remove them
        path.sort_by_key(|a| a.id);
        path.dedup_by_key(|a| a.id);

        // if path is complete, return
        if path.len() == nodes.len() { return; }

        // add missing nodes to path using Nearest Neighbor algorithm
        for node_idx in 0..nodes.len() {
            let node = &nodes[node_idx];

            // if node coordinates are already in path, skip it
            if path.iter().any(|n| n.id == node.id) { continue; }

            // else add it
            // find path node that is closest to current node
            let mut min_index = path.iter()
                .enumerate()
                .min_by_key(|(_, n)| dist(&node, n))
                .unwrap().0;

            // insert node in path
            // if node is closer to previous node than to next node, insert it before previous node
            if min_index == 0 {
                path.insert(0, node.clone());
            } else if min_index == path.len() - 1 {
                path.push(node.clone());
            } else {
                let dist_before = dist(&node, &path[min_index - 1]);
                let dist_after = dist(&node, &path[min_index + 1]);
                if dist_before < dist_after { min_index -= 1; }
                path.insert(min_index, node.clone());
            }
        }
    }
}

/// read input data from file
pub fn get_nodes_from_data(file_path: &str) -> Vec<Node> {
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
            Node {
                id: line.next().unwrap().parse().unwrap(),
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
            node.items.push(Item { profit, weight });
        });
    node_coordinates
}

/// calculate manhattan distance between 2 nodes
pub fn dist(node1: &Node, node2: &Node) -> i32 {
    let dx = (node1.coordinates.0 - node2.coordinates.0).abs();
    let dy = (node1.coordinates.1 - node2.coordinates.1).abs();
    dx + dy
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
