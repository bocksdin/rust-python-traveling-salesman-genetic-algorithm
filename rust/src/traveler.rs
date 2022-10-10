// EXTERNAL CRATES
use rand::Rng;
use rand::rngs::ThreadRng;

// INTERNAL CRATES
use crate::path::Path;


#[derive(Debug, Copy, Clone)]
pub struct Traveler {
    pub chromosome: [i8; 20],
    pub cost: f32,
    pub distance: i32,
    pub time: f32,
    pub generation: i32
}

impl Default for Traveler {
    fn default() -> Traveler {
        Traveler {
            chromosome: [0; 20],
            cost: f32::MAX,
            distance: 0,
            time: 0.,
            generation: 0
        }
    }
}

impl Traveler {
    pub fn init(&mut self, paths: &[Path; 20], chromosome: Option<&[i8; 20]>, generation: Option<i32>) {
        self.cost = 0.;
        self.distance = 0;
        self.time = 0.;
        
        match generation {
            None => { self.generation = 0 },
            Some(value) => { self.generation = value }
        }

        match chromosome {
            None => {
                let mut rng: ThreadRng = rand::thread_rng();
                for i in 0..paths.len() {
                    self.chromosome[i] = rng.gen_range(0..2);
                }
            },
            Some(value) => {
                self.chromosome = *value;
            }
        }
    }

    pub fn crossover(&self, other_traveler: &Traveler, paths: &[Path; 20]) -> (Traveler, Traveler) {
        let mut child1: [i8; 20] = [0; 20];
        let mut child2: [i8; 20] = [0; 20];

        for i in 0..self.chromosome.len() as usize {
            if i % 2 == 0 {
                child1[i] = self.chromosome[i];
                child2[i] = other_traveler.chromosome[i];
            } else {
                child1[i] = other_traveler.chromosome[i];
                child2[i] = self.chromosome[i];
            }
        }

        let (mut children1, mut children2): (Traveler, Traveler) = (
            Traveler { ..Default::default() },
            Traveler { ..Default::default() }
        );

        children1.init(paths, Some(&child1), Some(&self.generation + 1));
        children2.init(paths, Some(&child2), Some(&self.generation + 1));

        (children1, children2)
    }

    pub fn mutation(&mut self, rate: Option<f32>) {
        let usable_rate: f32;

        match rate {
            None => usable_rate = 0.01,
            Some(value) => usable_rate = value
        }

        let mut rng = rand::thread_rng();
        for i in 0..self.chromosome.len() {
            if rng.gen_range(0.0..1.0) > usable_rate {
                if self.chromosome[i] == 0 {
                    self.chromosome[i] = 1;
                } else {
                    self.chromosome[i] = 0;
                }
            }
        }
    }

    pub fn fitness(&mut self, cities: &[i8; 5], paths: &[Path; 20]) {
        if self.chromosome.iter().filter(|x| x == &&(1 as i8)).count() != cities.len() {
            self.cost = f32::MAX;
            return;
        }

        let (mut cost, mut distance, mut time): (f32, i32, f32) = (0., 0, 0.);
        let mut origins_visited: Vec<i8> = vec![];
        let mut destinations_visited: Vec<i8> = vec![];
        let mut combinations: Vec<(i8, i8)> = vec![];

        for i in 0..self.chromosome.len() {
            if self.chromosome[i] == 1 {
                let curr: Path = paths[i];
                origins_visited.push(curr.origin);
                destinations_visited.push(curr.destination);
                combinations.push((curr.origin, curr.destination));
                cost += curr.travel_time;
                distance += curr.distance;
                time += curr.travel_time;
            }
        }

        self.distance = distance;
        self.time = time;

        if self.cost < f32::MAX {
            for city in cities {
                let origin_count = origins_visited.iter().filter(|x| x == &city).count();
                let destination_count = destinations_visited.iter().filter(|x| x == &city).count();
                if origin_count != 1 || destination_count != 1 {
                    self.cost = f32::MAX;
                    return;
                }
            }
        }

        self.cost = cost;
    }
}