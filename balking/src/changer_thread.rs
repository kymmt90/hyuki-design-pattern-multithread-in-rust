use std::sync::Arc;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::data::Data;

pub struct ChangerThread {
    data: Arc<Data>,
    rng: SmallRng,
}

impl ChangerThread {
    pub fn new(data: Arc<Data>) -> Self {
        Self {
            data,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn run(&mut self) {
        for i in 0.. {
            self.data.change(&format!("No.{}", i));

            std::thread::sleep(std::time::Duration::from_millis(
                self.rng.gen_range(0..=1000),
            ));

            self.data.save();
        }
    }
}
