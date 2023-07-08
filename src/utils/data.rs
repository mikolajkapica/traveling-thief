use std::fs;
use super::structs::{Node, Item};

/// read input data from file and return nodes with items
pub fn parse_to_nodes_from(file_path: &str) -> Vec<Node> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    // skip information
    while !lines.next().unwrap().contains("NODE_COORD_SECTION") {}

    // Get node coordinates using iterator
    let mut node_coordinates: Vec<Node> = lines.clone()
        .take_while(|line| !line.starts_with("ITEMS SECTION"))
        .map(|line| {
            let mut line = line.split_whitespace();
            let id = line.next().unwrap().parse().unwrap();
            let coordinates = (line.next().unwrap().parse().unwrap(), 
                               line.next().unwrap().parse().unwrap()); Node {id, coordinates, items: Vec::new()}
        }).collect();

    // Get items
    lines
        .skip(node_coordinates.len() + 1)
        .for_each(|line| {
            let mut line = line.split_whitespace();
            let id = line.next().unwrap().parse().unwrap();
            let profit = line.next().unwrap().parse().unwrap();
            let weight = line.next().unwrap().parse().unwrap();
            let node_id = line.next().unwrap().parse::<usize>().unwrap() - 1;
            node_coordinates.get_mut(node_id).unwrap().items.push(Item {id, profit, weight});
        });

    // return nodes with items
    node_coordinates
}
