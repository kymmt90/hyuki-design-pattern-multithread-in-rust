use std::{sync::Arc, thread};

use crate::data::Data;

pub struct ReaderThread {
    data: Arc<Data>,
}

impl ReaderThread {
    pub fn new(data: Arc<Data>) -> Self {
        Self { data }
    }

    pub fn run(&self) {
        loop {
            let s = self.data.read().iter().collect::<String>();
            println!("{}: {}", thread::current().name().unwrap(), s);
        }
    }
}
