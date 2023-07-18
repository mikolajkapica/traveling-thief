mod utils;
use utils::structs::Settings;
use utils::data::parse_to_nodes_from;
use utils::chromosome::Chromosome;
use utils::debug::print_progress;

use rand::Rng;

mod tests;

fn main() {
    // initialize parameters
    let settings = Settings {
        number_of_generations: std::env::args().nth(1).unwrap_or("1000".to_string()).parse::<i32>().unwrap(),
        population_size: 1000,
        item_chance: 0.1,
        mutation_rate: 0.05,
        tournament_size: 100,
        elitism: true,
        elitism_size: 100,
        renting_rate: 72.70,
        v_max: 10.0,
        v_min: 0.1,
        maximum_weight: 637010,
        data_path: "src/a280_n1395_uncorr-similar-weights_05.ttp.txt",
    };

    // destructuring
    let Settings { number_of_generations, population_size, tournament_size, elitism, elitism_size, data_path, .. } = settings;

    // random number generator
    let mut rng = &mut rand::thread_rng();

    // import data
    let nodes = parse_to_nodes_from(data_path);

    // create initial population
    let mut population: Vec<Chromosome> = (0..population_size)
        .map(|_| Chromosome::new(&nodes, rng, &settings))
        .collect();
   
    // evolve
    for generation in 0..number_of_generations {
        print_progress(generation, &population);

        // create new population
        let mut new_population = Vec::new();
   
        // elitism
        if elitism {
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            new_population = population[0..elitism_size].to_vec();
        }
   
        // fill new population with children
        while new_population.len() != population_size {
            // get 2 winning parents from tournament
            let mut tournament = Vec::new();
            for _ in 0..tournament_size { tournament.push(&population[rng.gen_range(0..population_size)]); }
            tournament.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            let parent1 = tournament[0].clone();
            let parent2 = tournament[1].clone();
   
            // crossover and mutate parents to create child
            let (start, end) = random_subpart(parent1.path.len(), &mut rng); 
            let mut child = parent1.crossover(&parent2, start, end, &settings);

            child.mutate(&nodes, &mut rng, &settings);
            new_population.push(child);
        }
   
        // replace old population with new population
        population = new_population;
    }
   
    // get best gene
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    let best_gene = population[0].clone();
   
    // print best gene
    // println!("\nPath: {}", best_gene.path.iter().map(|node| format!("({},{})", node.coordinates.0, node.coordinates.1)).collect::<Vec<String>>().join(" -> "));
    println!("Fitness: {}", best_gene.fitness);
}

/// get random subpart of path
fn random_subpart(path_length: usize, rng: &mut rand::rngs::ThreadRng) -> (usize, usize) {
    let start = rng.gen_range(0..path_length);
    let end = rng.gen_range(0..path_length);
    if start < end {
        (start, end)
    } else {
        (end, start)
    }
}
