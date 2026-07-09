use alloc::vec;

use crate::library::random::Random;

pub fn random_with_limit_demo(limit: usize, seed: usize, number_of_draws: usize, number_of_repetions: usize) {
    println!("\nDEMO FOR RANDOM NUMBER GENERATION, LIMITED TO <= {}.\n", limit);
    for reps in 0..number_of_repetions {
        let mut rng = Random::new(seed);
        let mut count = vec![0; limit];
        for i in 0..number_of_draws {
            let n = rng.random_limited(limit - 1);
            count[n] += 1;
        }
        println!("Counters for seed {}: {:?}", rng.get_seed(), count);

        let mut average_deviation = 0;
        let ideal = number_of_draws / limit;
        for i in 0..(limit - 1) {
            if count[i] > ideal {
                average_deviation += count[i] - ideal;
            }
            else {
                average_deviation += ideal - count[i];
            }
        }
        average_deviation = average_deviation / limit;
        println!("Average deviation (from {}): {}\n", ideal, average_deviation);
    }
    println!("END :)\n");
}

pub fn random_with_range_demo(min: usize, max: usize, seed: usize, number_of_draws: usize, number_of_repetions: usize) {
    println!("\nDEMO FOR RANDOM NUMBER GENERATION, IN RANGE FROM {} TO {}.\n", min, max);
    for reps in 0..number_of_repetions {
        let mut rng = Random::new(seed);
        let mut count = vec![0; max];
        for i in 0..number_of_draws {
            let n = rng.random_in_range(min,max - 1);
            count[n] += 1;
        }
        println!("Counters for seed {}: {:?}", rng.get_seed(), count);
        
        let mut average_deviation = 0;
        let ideal = number_of_draws / (max - min);
        for i in min..(max - 1) {
            if count[i] > ideal {
                average_deviation += count[i] - ideal;
            }
            else {
                average_deviation += ideal - count[i];
            }
        }
        average_deviation = average_deviation / (max - min);
        println!("Average deviation (from {}): {}\n", ideal, average_deviation);
    }
    println!("END :)\n");
}