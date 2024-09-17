use std::{sync::Arc, thread, time::Duration};

use rand::{rngs::SmallRng, Rng as _, SeedableRng as _};

use crate::{channel::Channel, request::Request};

pub struct ClientThread {
    name: String,
    channel: Arc<Channel>,
    rng: SmallRng,
}

impl ClientThread {
    pub fn new(name: String, channel: Arc<Channel>) -> Self {
        Self {
            name,
            channel,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn start(&mut self) {
        for i in 0.. {
            let request = Request::new(self.name.clone(), i);
            self.channel.put_request(request);

            thread::sleep(Duration::from_millis(self.rng.gen_range(0..=1000)));
        }
    }
}
