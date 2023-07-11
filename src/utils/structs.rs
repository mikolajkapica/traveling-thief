#[derive(Clone, Default, Debug)]
pub struct Node {
    pub id: i32,
    pub coordinates: (i32, i32),
    pub items: Vec<Item>,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Node {
    /// Calculate manhattan distance between two nodes
    pub fn distance_to(&self, other: &Node) -> i32 {
        let dx = (self.coordinates.0 - other.coordinates.0).abs();
        let dy = (self.coordinates.1 - other.coordinates.1).abs();
        dx + dy
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id : i32,
    pub profit: i32,
    pub weight: i32,
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

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

