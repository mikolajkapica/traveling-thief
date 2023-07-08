use crate::Chromosome;

pub fn print_progress(generation: i32, population: &Vec<Chromosome>) {
        // print!("\x1B[2J\x1B[1;1H"); // clear screen
        println!("Generation: {} | Best fitness: {}", generation, population[0].fitness); // print progress
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
        for node in &population[0].path {
            print!("In node {} thief picked up ", node.id);
            if node.items.len() > 0 {
                print!("{} items", node.items.len());
            } else {
                print!("0 items");
            }
            print!("\n-> ");
        }
}
