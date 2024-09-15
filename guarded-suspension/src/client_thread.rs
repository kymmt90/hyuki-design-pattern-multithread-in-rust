use std::{sync::Arc, thread};

use rand::prelude::*;

use crate::request::{Request, RequestQueue};

pub struct ClientThread {
    request_queue: Arc<RequestQueue>,
    rng: SmallRng,
}

impl ClientThread {
    pub fn new(request_queue: Arc<RequestQueue>, seed: u64) -> Self {
        Self {
            request_queue,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn start(&mut self) {
        for i in 0..10000 {
            let request = Request::new(format!("No. {}", i));
            println!("{} requests {}", thread::current().name().unwrap(), request);
            self.request_queue.put_request(request);

            std::thread::sleep(std::time::Duration::from_millis(
                self.rng.gen_range(0..=1000),
            ));
        }
    }
}
