use std::{thread, time::Duration};

use crate::log;

pub struct ClientThread {
    name: String,
}

impl ClientThread {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn run(&self) {
        println!("{} BEGIN", self.name);

        for i in 0..10 {
            log::println(format!("i = {}\n", i).as_str());

            thread::sleep(Duration::from_millis(100));
        }

        println!("{} END", self.name);
    }
}
