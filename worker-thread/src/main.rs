mod channel;
mod client_thread;
mod request;
mod worker_thread;

use std::{sync::Arc, thread};

use channel::Channel;
use client_thread::ClientThread;

fn main() {
    let channel = Arc::new(Channel::new(5));

    Channel::start_workers(channel.clone());

    thread::scope(|s| {
        for name in ["Alice", "Bobby", "Chris"] {
            s.spawn({
                let channel = channel.clone();
                move || {
                    ClientThread::new(name.into(), channel).start();
                }
            });
        }
    });
}
