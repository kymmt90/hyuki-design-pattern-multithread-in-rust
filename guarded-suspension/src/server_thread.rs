use std::{sync::Arc, thread};

use rand::prelude::*;

use crate::request::RequestQueue;

pub struct ServerThread {
    request_queue: Arc<RequestQueue>,
    rng: SmallRng,
}

impl ServerThread {
    pub fn new(request_queue: Arc<RequestQueue>, seed: u64) -> Self {
        Self {
            request_queue,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn start(&mut self) {
        for _ in 0..10000 {
            let request = self.request_queue.get_request();
            println!("{} handles {}", thread::current().name().unwrap(), request);

            std::thread::sleep(std::time::Duration::from_millis(
                self.rng.gen_range(0..=1000),
            ));
        }
    }
}
