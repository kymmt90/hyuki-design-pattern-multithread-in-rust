use std::{thread, time::Duration};

use crate::active_object::ArcActiveObject;

pub struct DisplayClientThread {
    active_object: ArcActiveObject,
}

impl DisplayClientThread {
    pub fn new(active_object: ArcActiveObject) -> Self {
        Self { active_object }
    }

    pub fn run(&self) {
        for i in 0.. {
            let string = format!("{} {}", thread::current().name().unwrap(), i);

            self.active_object.display_string(&string);

            thread::sleep(Duration::from_millis(200));
        }
    }
}
