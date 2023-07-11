#[cfg(test)]
mod tests {
    use crate::utils::structs::{Settings, Node};
    use crate::utils::chromosome::Chromosome;

    #[test]
    fn test_crossover() {
        // initialize parameters
        let settings = Settings {
            number_of_generations: std::env::args().nth(1).unwrap_or("1000".to_string()).parse::<i32>().unwrap(),
            population_size: 250,
            item_chance: 0.1,
            mutation_rate: 0.1,
            tournament_size: 5,
            elitism: true,
            elitism_size: 5,
            renting_rate: 0.2,
            v_max: 10.0,
            v_min: 0.1,
            maximum_weight: 1000000,
            data_path: "src/a280_n1395_uncorr-similar-weights_05.ttp.txt",
        };
        
        // mock two chromosomes 
        let mut nodes1 = vec![Node::default(); 8];
        nodes1[0].id = 3;
        nodes1[1].id = 7;
        nodes1[2].id = 1;
        nodes1[3].id = 4;
        nodes1[4].id = 6;
        nodes1[5].id = 5;
        nodes1[6].id = 2;
        nodes1[7].id = 8;
        let chromosome1 = Chromosome {
            path: nodes1,
            fitness: 0.0,
        };

        let mut nodes2 = vec![Node::default(); 8];
        nodes2[0].id = 1;
        nodes2[1].id = 8;
        nodes2[2].id = 6;
        nodes2[3].id = 4;
        nodes2[4].id = 5;
        nodes2[5].id = 2;
        nodes2[6].id = 3;
        nodes2[7].id = 7;
        let chromosome2 = Chromosome {
            path: nodes2,
            fitness: 0.0,
        };

        // perform crossover
        let (start, end) = (4, 6);
        let child = chromosome1.clone().crossover(&chromosome2.clone(), start, end, &settings);

        // check if child is correct
        let mut expected_child = vec![Node::default(); 8];
        expected_child[0].id = 1;
        expected_child[1].id = 8;
        expected_child[2].id = 3;
        expected_child[3].id = 4;
        expected_child[4].id = 6;
        expected_child[5].id = 5;
        expected_child[6].id = 2;
        expected_child[7].id = 7;
        
        assert_eq!(child.path, expected_child);

        // and in reverse
        // perform crossover
        let (start, end) = (4, 6);
        let child = chromosome2.clone().crossover(&chromosome1.clone(), start, end, &settings);

        // check if child is correct
        let mut expected_child = vec![Node::default(); 8];
        expected_child[0].id = 6;
        expected_child[1].id = 7;
        expected_child[2].id = 1;
        expected_child[3].id = 4;
        expected_child[4].id = 5;
        expected_child[5].id = 2;
        expected_child[6].id = 3;
        expected_child[7].id = 8;
        
        assert_eq!(child.path, expected_child);
    }
}
