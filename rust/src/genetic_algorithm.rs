use rand::Rng;
use rand::rngs::ThreadRng;
use indicatif::ProgressBar;

// INTERNAL CRATES
use crate::path::Path;
use crate::traveler::Traveler;


#[derive(Debug)]
pub struct GeneticAlgorithm {
    pub population_size: i32,
    pub population: [Traveler; 200],
    pub generation: i32,
    pub best_solution: Option<Traveler>
}

impl Default for GeneticAlgorithm {
    fn default() -> GeneticAlgorithm {
        GeneticAlgorithm {
            population_size: 0,
            population: [Traveler { ..Default::default() }; 200],
            generation: 0,
            best_solution: None
        }
    }
}

impl GeneticAlgorithm {
    pub fn initialize_population(&mut self, population_size: i32, paths: &[Path; 20], cities: &[i8; 5]) {
        self.population_size = population_size;

        for i in 0..self.population_size as usize {
            self.population[i].init(paths, None, None);
        }

        self.calculate_fitness(cities, paths);
        self.order_population();

        self.best_solution = Some(self.population[0].clone());
    }

    fn calculate_fitness(&mut self, cities: &[i8; 5], paths: &[Path; 20]) {
        for i in 0..self.population.len() {
            self.population[i].fitness(cities, paths);
        }
    }

    fn order_population(&mut self) {
        self.population.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
    }

    fn best_traveler(&mut self) {
        if self.population[0].cost < self.best_solution.as_ref().unwrap().cost {
            self.best_solution = Some(self.population[0].clone());
        }
    }

    fn sum_costs(&self) -> f64 {
        let mut sum: f64 = 0.;

        for i in 0..self.population.len() {
            sum += self.population[i].cost as f64;
        }

        sum
    }

    fn select_parent_cutoff(&self, sum_costs: f64) -> usize {
        let mut cutoff: isize = -1;
        let mut rng: ThreadRng = rand::thread_rng();
        let random_value: f64 = rng.gen_range(0.0..sum_costs);

        let mut sum: f64 = 0.;
        let mut i: usize = 0;
        while i < self.population.len() && sum < random_value {
            sum += self.population[i].cost as f64;
            cutoff += 1;
            i += 1;
        }

        cutoff as usize
    }

    pub fn solve(&mut self, mutation_probability: Option<f32>, number_of_generations: i32, population_size: i32, paths: &[Path; 20], cities: &[i8; 5]) -> Option<&Traveler> {
        self.initialize_population(population_size, paths, cities);
        let pb: ProgressBar = indicatif::ProgressBar::new(number_of_generations as u64);

        for _i in 0..number_of_generations {
            let sum: f64 = self.sum_costs();

            let mut new_population: [Traveler; 200] = [Traveler { ..Default::default() }; 200];
            for j in (0..self.population_size as usize).step_by(2) {
                let parents: [usize; 2] = [self.select_parent_cutoff(sum), self.select_parent_cutoff(sum)];
                let (mut child1, mut child2): (Traveler, Traveler) = self.population[parents[0]].crossover(&self.population[parents[1]], paths);

                child1.mutation(mutation_probability);
                child2.mutation(mutation_probability);

                new_population[j] = child1;
                new_population[j + 1] = child2;
            }

            self.population = new_population;
            self.calculate_fitness(cities, paths);
            self.order_population();
            self.best_traveler();
            self.generation += 1;
            pb.inc(1);
        }

        self.best_solution.as_ref()
    }
}