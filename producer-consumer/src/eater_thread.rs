use std::{sync::Arc, thread, time::Duration};

use rand::{rngs::SmallRng, Rng as _, SeedableRng as _};

use crate::table::Table;

pub struct EaterThread {
    table: Arc<Table>,
    rng: SmallRng,
}

impl EaterThread {
    pub fn new(table: Arc<Table>, seed: u64) -> Self {
        Self {
            table,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn start(&mut self) {
        loop {
            self.table.take();

            thread::sleep(Duration::from_millis(self.rng.gen_range(0..=1000)));
        }
    }
}
