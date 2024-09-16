use std::{sync::Arc, thread, time::Duration};

use rand::{rngs::SmallRng, Rng as _, SeedableRng as _};

use crate::data::Data;

pub struct WriterThread {
    data: Arc<Data>,
    filler: String,
    rng: SmallRng,
    index: usize,
}

impl WriterThread {
    pub fn new(data: Arc<Data>, filler: String) -> Self {
        Self {
            data,
            filler,
            rng: SmallRng::from_entropy(),
            index: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let c = self.nextchar();
            self.data.write(c);

            thread::sleep(Duration::from_millis(self.rng.gen_range(0..=1000)));
        }
    }

    fn nextchar(&mut self) -> char {
        let c = self.filler.chars().nth(self.index).unwrap();
        self.index = (self.index + 1) % self.filler.len();

        c
    }
}
