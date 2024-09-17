use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::channel::Channel;

pub struct WorkerThread {
    name: String,
}

impl WorkerThread {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn start(&self, channel: Arc<Channel>) -> JoinHandle<()> {
        thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || loop {
                let mut request = channel.take_request();
                request.execute();
            })
            .unwrap()
    }
}
