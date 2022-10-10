// EXTERNAL CRATES
use std::time::{self, Duration};

// INTERNAL CRATES
use crate::path::Path;
use crate::genetic_algorithm::GeneticAlgorithm;

// BRING INTO PROJECT
pub mod path;
pub mod traveler;
pub mod genetic_algorithm;


fn main() {
    let cities: [i8; 5] = [1, 2, 3, 4, 5];
    let paths: [Path; 20] = [
        // OPTIMAL
        Path { origin: 1, destination: 2, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 2, destination: 3, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 3, destination: 4, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 4, destination: 5, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 5, destination: 1, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 2, destination: 1, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 3, destination: 2, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 4, destination: 3, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 5, destination: 4, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
        Path { origin: 1, destination: 5, distance: 10, avg_speed: 60, travel_time: 10. / 60. },
    
        // SUBOPTIMAL
        Path { origin: 1, destination: 3, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 1, destination: 4, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 2, destination: 4, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 2, destination: 5, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 3, destination: 1, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 3, destination: 5, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 4, destination: 1, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 4, destination: 2, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 5, destination: 2, distance: 20, avg_speed: 60, travel_time: 20. / 60. },
        Path { origin: 5, destination: 3, distance: 20, avg_speed: 60, travel_time: 20. / 60. }
    ];


    
    const MUTATION_PROBABILITY: Option<f32> = Some(0.01);
    const NUMBER_OF_GENERATIONS: i32 = 1000;
    const POPULATION_SIZE: i32 = 200;

    let mut times: Vec<Duration> = vec![];
    let mut best_result: f32  = f32::MAX;
    let mut results: Vec<f32> = vec![];
    for _i in 0..10000 {
        let mut test_ga = GeneticAlgorithm { ..Default::default() };
        let start = time::Instant::now();
        let result = test_ga.solve(MUTATION_PROBABILITY, NUMBER_OF_GENERATIONS, POPULATION_SIZE, &paths, &cities);
        let end = time::Instant::now();
        times.push(end - start);

        match result {
            None => {},
            Some(value) => {
              if best_result > value.cost {
                best_result = value.cost;
              }

              results.push(value.cost);
            }
        }
    }

    println!("\n\n\nBest Result: {}, Avg Result: {}, in average of {:?}, total time of {:?}\n\n\n",
            best_result,
            results.iter().sum::<f32>() / results.len() as f32,
            times.iter().sum::<Duration>() / times.len() as u32,
            times.iter().sum::<Duration>());
}