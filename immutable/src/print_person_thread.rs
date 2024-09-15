use std::{sync::Arc, thread};

use crate::person::Person;

pub struct PrintPersonThread {
    person: Arc<Person>,
}

impl PrintPersonThread {
    pub fn new(person: Arc<Person>) -> Self {
        Self { person }
    }

    pub fn run(&self) {
        loop {
            println!(
                "{} prints {}",
                thread::current().name().unwrap(),
                self.person
            );
        }
    }
}
