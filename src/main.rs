use std::{env, io};
use std::io::Write;
use rand::{Rng, thread_rng};
use ttp::{Chromosome, get_nodes_from_data, Settings};

fn main() {
    // initialize parameters
    let settings = Settings {
        number_of_generations: env::args().nth(1).unwrap_or("1000".to_string()).parse::<i32>().unwrap(),
        population_size: 250,
        item_chance: 0.2,
        mutation_rate: 0.1,
        tournament_size: 5,
        elitism: true,
        elitism_size: 5,
        renting_rate: 0.2,
        v_max: 10.0,
        v_min: 0.1,
        maximum_weight: 10,
        data_path: "src/a280_n1395_uncorr-similar-weights_05.ttp.txt",
    };

    // destructuring
    let Settings { number_of_generations, population_size, tournament_size, elitism, elitism_size, data_path, .. } = settings;

    // random number generator
    let mut rng = &mut thread_rng();

    // import data
    let nodes = get_input_data("src/a280_n1395_uncorr-similar-weights_05.ttp.txt");

    // create initial population
    let mut population: Vec<Chromosome> = (0..population_size)
        .map(|_| Chromosome::new(&nodes))
        .collect();

    // evolve
    let mut generation = 0;
    while generation < max_generations {
        generation += 1;
        let mut new_population = Vec::new();
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        if elitism {

            for i in 0..elitism_size {
                new_population.push(population[i].clone());
            }
        }
        while new_population.len() != population_size {
            let mut tournament = Vec::new();
            for _ in 0..tournament_size {
                tournament.push(population[rng.gen_range( 0..population_size)].clone());
            }
            tournament.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            let parent1 = tournament[0].clone();
            let parent2 = tournament[1].clone();
            let mut child = parent1.crossover(&parent2, &nodes);
            child.mutate(mutation_rate, &nodes);
            new_population.push(child);
        }
        population = new_population;
        print!("\rGeneration: {} ", generation);
    }
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    println!("Path: \n{}", population[0].path.iter().map(|node| format!("{},{}", node.coordinates.0, node.coordinates.1)).collect::<Vec<String>>().join("\n"));

    println!("Fitness: {}", population[0].fitness);
}
