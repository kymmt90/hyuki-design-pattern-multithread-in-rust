use std::{fmt::Display, thread, time::Duration};

use rand::{rngs::SmallRng, Rng as _, SeedableRng as _};

pub struct Request {
    name: String,
    number: i32,
    rng: SmallRng,
}

impl Request {
    pub fn new(name: String, number: i32) -> Request {
        Self {
            name,
            number,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn execute(&mut self) {
        println!("{} executes {}", thread::current().name().unwrap(), self);

        thread::sleep(Duration::from_millis(self.rng.gen_range(0..=1000)));
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ Request from {} No.{} ]", self.name, self.number)
    }
}
