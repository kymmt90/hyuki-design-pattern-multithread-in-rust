use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use rand::{rngs::SmallRng, Rng as _, SeedableRng as _};

use crate::table::Table;

static ID: AtomicUsize = AtomicUsize::new(0);

pub struct MakerThread {
    table: Arc<Table>,
    rng: SmallRng,
}

impl MakerThread {
    pub fn new(table: Arc<Table>, seed: u64) -> Self {
        Self {
            table,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(Duration::from_millis(self.rng.gen_range(0..=1000)));

            let cake = format!(
                "[ Cake No.{} by {} ]",
                self.next_id(),
                thread::current().name().unwrap()
            );
            self.table.put(cake);
        }
    }

    fn next_id(&self) -> usize {
        ID.fetch_add(1, Ordering::SeqCst)
    }
}
