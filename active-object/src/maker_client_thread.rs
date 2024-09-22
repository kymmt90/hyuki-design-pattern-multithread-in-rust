use std::{thread, time::Duration};

use crate::active_object::ArcActiveObject;

pub struct MakerClientThread {
    active_object: ArcActiveObject,
    fillchar: char,
}

impl MakerClientThread {
    pub fn new(active_object: ArcActiveObject, name: &str) -> Self {
        Self {
            active_object,
            fillchar: name.chars().next().unwrap(),
        }
    }

    pub fn run(&self) {
        for i in 0.. {
            let result = self.active_object.make_string(i, self.fillchar);

            thread::sleep(Duration::from_millis(10));

            let value = result.get_result_value();

            println!("{}: value = {}", thread::current().name().unwrap(), value);
        }
    }
}
