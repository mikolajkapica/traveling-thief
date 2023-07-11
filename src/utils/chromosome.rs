use super::structs::{Node, Settings};
use rand::Rng;
use rand::rngs::ThreadRng;

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
        let mut path: Vec<Node> = (0..nodes.len())
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

        // get path fitness (path might be changed if weight is more than maximum weight)
        let fitness = Chromosome::calculate_fitness(&mut path, settings);

        // create chromosome
        Chromosome {
            path,
            fitness,
        }
    }

    /// calculate fitness of path - fitness = sum of profits of all items - (sum of distances between nodes / (v_max - (nu * current_weight)))
    fn calculate_fitness(path: &mut Vec<Node>, settings: &Settings) -> f32 {
        let Settings { renting_rate, v_max, v_min, maximum_weight, .. } = settings;
        let nu = (v_max-v_min) / *maximum_weight as f32;

        let path_for_stats = path.clone();

        // initialize fitness
        let mut fitness = 0.0;

        // add profit of all items
        for node in &path_for_stats {
            for item in &node.items {
                fitness += item.profit as f32;
            }
        }

        // while going through path,
        // thief travels distance, more distance = less fitness
        // thief carries more weight, more weight = less fitness
        let mut current_weight = 0;
        let mut last_node = &path_for_stats[0];
        let mut subtrahend = 0.0;
        for node in &path_for_stats {
            // add weight of items in current node to current weight
            for item in &node.items {
                current_weight += item.weight;
            }

            // calculate subtrahend
            subtrahend += node.distance_to(&last_node) as f32 / (v_max - (nu * current_weight as f32));
            last_node = node;
        }

        if current_weight > *maximum_weight {
            // if current weight is more than maximum weight, delete some of its items, and
            // calculate fitness once more

            // delete items from nodes until current weight is less than maximum weight
            for i in 0..path.len() {
                if current_weight <= *maximum_weight {
                    break;
                }
                while current_weight > *maximum_weight && path[i].items.len() > 0 {
                    let item = path[i].items.pop().unwrap();
                    current_weight -= item.weight;
                }
            }

            Self::calculate_fitness(path, settings);
        }

        // multiply subtrahend by renting rate
        subtrahend *= renting_rate;

        // subtract subtrahend from fitness
        fitness -= subtrahend;

        fitness
    }

    /// make child from 2 parents by combining parts of both parents
    pub fn crossover(self, other: &Chromosome, start: usize, end: usize, settings: &Settings) -> Chromosome {
        let path_length = self.path.len();

        // create child path
        let mut child_path = vec![Node::default(); path_length];

        // fill in nodes from parent1
        child_path[start..=end].clone_from_slice(&self.path[start..=end]);

        // PMX
        for i in start..=end {
            if !child_path.contains(&other.path[i]) {
                let node = &other.path[i];
                let mut pos_to_replace = node.id;
                loop {
                    let index_of_node = other.path.iter().position(|n| n.id == pos_to_replace).unwrap();

                    if index_of_node < start || index_of_node > end {
                        child_path[index_of_node] = node.clone();
                        break;
                    } else {
                         pos_to_replace = self.path[index_of_node].id;
                    }
                }
            }
        }

        for i in 0..path_length {
            if child_path[i].id == 0 {
                child_path[i] = other.path[i].clone();
            }
        }

        // calculate fitness
        let fitness = Chromosome::calculate_fitness(&mut child_path, settings);

        Chromosome {
            path: child_path,
            fitness,
        }
    }

    /// swap 2 random nodes in path
    pub fn mutate(&mut self, nodes: &Vec<Node>, rng: &mut ThreadRng, settings: &Settings) {
        let Settings { mutation_rate, .. } = settings;

        for i in 0..self.path.len() {
            // find node with the same id as path[i]
            let node = nodes.iter().find(|n| n.id == self.path[i].id).unwrap();

            // add or remove random items from node
            for item in &node.items {
                if rng.gen_bool(*mutation_rate as f64) {
                    if self.path[i].items.contains(item) {
                        self.path[i].items.retain(|x| x != item);
                    } else {
                        self.path[i].items.push(item.clone());
                    }
                }
            }

            // swap 2 random nodes in path
            if rng.gen_bool(*mutation_rate as f64) {
                let index = rng.gen_range(0..self.path.len());
                self.path.swap(i, index);
            }
        }

        // recalculate fitness
        self.fitness = Chromosome::calculate_fitness(&mut self.path, settings);
    }

//    ///repair path by adding missing cities using Nearest Neighbor algorithm
//     fn repair(path: &mut Vec<Node>, nodes: &Vec<Node>) {
//         // if we have duplicate nodes in path, remove them
//         path.sort_by_key(|a| a.id);
//         path.dedup_by_key(|a| a.id);
//
//         // if path is complete, return
//         if path.len() == nodes.len() { return; }
//
//         // add missing nodes to path using Nearest Neighbor algorithm
//         for node_idx in 0..nodes.len() {
//             let node = &nodes[node_idx];
//
//             // if node coordinates are already in path, skip it
//             if path.iter().any(|n| n.id == node.id) { continue; }
//
//             // else add it
//             // find path node that is closest to current node
//             let mut min_index = path.iter()
//                 .enumerate()
//                 .min_by_key(|(_, n)| node.distance_to(n))
//                 .unwrap().0;
//
//             // insert node in path
//             // if node is closer to previous node than to next node, insert it before previous node
//             if min_index == 0 {
//                 path.insert(0, node.clone());
//             } else if min_index == path.len() - 1 {
//                 path.push(node.clone());
//             } else {
//                 let dist_before = node.distance_to(&path[min_index - 1]);
//                 let dist_after = node.distance_to(&path[min_index + 1]);
//                 if dist_before < dist_after { min_index -= 1; }
//                 path.insert(min_index, node.clone());
//             }
//         }
//     }
}

