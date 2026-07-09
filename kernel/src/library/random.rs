use crate::device::pit;

// A few default seeds that generally work.
pub const DEFAULT_SEED_1: usize = 1009;
pub const DEFAULT_SEED_2: usize = 2351;
pub const DEFAULT_SEED_3: usize = 4271;
pub const DEFAULT_SEED_4: usize = 6563;
pub const DEFAULT_SEED_5: usize = 7919;

/// A (pseudo) random number generator which uses the system time.
pub struct Random {
    seed: usize,
    prev: usize,
}

impl Random {
    /// Returns a new random number generator with the given seed (use big prime numbers for best results).
    pub fn new (seed: usize) -> Self {
        Self {seed: seed, prev: 3}
    }

    /// Generates a random number
    pub fn random(&mut self) -> usize {
        // Calcualtion: system time * seed + previous random number
        let result = pit::system_time().checked_mul(self.seed).unwrap_or(self.prev) + self.prev;
        self.prev = result;
        return result;
    }

    /// Generates a random number between 0 and the given limit.
    pub fn random_limited(&mut self, limit: usize) -> usize {
        self.random() % (limit + 1)
    }

    /// Generates a random number between the given bounds.
    pub fn random_in_range(&mut self, min: usize, max: usize) -> usize {
        self.random_limited(max - min) + min
    }

    /// Return the current seed.
    pub fn get_seed(&self) -> usize {
        return self.seed;
    }

    /// Modify the seed of a given random number generator.
    pub fn set_seed(&mut self, seed: usize) {
        self.seed = seed;
    }
}

