use crate::Chromosome;

pub fn print_progress(generation: i32, population: &Vec<Chromosome>) {
    // print!("\x1B[2J\x1B[1;1H"); // clear screen
    // avg fitness
    let mut avg_fitness = 0.0;
    for chromosome in population {
        avg_fitness += chromosome.fitness;
    }
    avg_fitness /= population.len() as f32;

    println!("Generation: {} | Best fitness: {} | Avg fitness: {}", generation, population[0].fitness, avg_fitness);


    // get average of all items profits
    // for chromosome in population {
    //     let mut items_count = 0;
    //     let mut avg_profit = 0.0;
    //     for node in &chromosome.path {
    //         for item in &node.items {
    //             items_count += 1;
    //             avg_profit += item.profit as f32;
    //         }
    //     }
    //     avg_profit /= items_count as f32;
    //     println!("Avg profit: {}", avg_profit);
    // }
    // all items count in population
    // let mut items_count = 0;
    // for chromosome in population {
    //     for node in &chromosome.path {
    //         items_count += node.items.len();
    //     }
    // }
    // println!("Items count: {}", items_count);
    // println!("Nodes count: {}", population[0].path.len());
    // print path with item if it has one
    // for node in &population[0].path {
    //     print!("In node {} thief picked up ", node.id);
    //     if node.items.len() > 0 {
    //         print!("{} items", node.items.len());
    //     } else {
    //         print!("0 items");
    //     }
    //     print!("\n-> ");
    // }
}
