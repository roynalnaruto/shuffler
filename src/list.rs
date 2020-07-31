use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

pub struct RandomisableList {
    rng: ChaCha20Rng,
    list: Vec<usize>,
}

impl RandomisableList {
    pub fn new(n: usize) -> Self {
        let rng = ChaCha20Rng::seed_from_u64(0u64);
        Self {
            rng: rng,
            list: (1..n).collect::<Vec<usize>>(),
        }
    }

    pub fn shuffle(&mut self, seed: [u8; 32]) {
        let mut rng = ChaCha20Rng::from_seed(seed);
        self.list.shuffle(&mut rng);
        self.rng = rng;
    }

    pub fn list(&self) -> Vec<usize> {
        self.list.clone()
    }
}
