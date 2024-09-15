use std::sync::Arc;

use crate::data::Data;

pub struct SaverThread {
    data: Arc<Data>,
}

impl SaverThread {
    pub fn new(data: Arc<Data>) -> Self {
        Self { data }
    }

    pub fn run(&self) {
        loop {
            self.data.save();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
