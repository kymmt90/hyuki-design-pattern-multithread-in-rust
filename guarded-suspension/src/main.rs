mod client_thread;
mod request;
mod server_thread;

use std::{sync::Arc, thread};

use client_thread::ClientThread;
use request::RequestQueue;
use server_thread::ServerThread;

fn main() {
    let queue = Arc::new(RequestQueue::new());

    thread::scope(|s| {
        thread::Builder::new()
            .name("Alice".into())
            .spawn_scoped(s, {
                let queue = queue.clone();
                || {
                    ClientThread::new(queue, 3141592).start();
                }
            })
            .unwrap();

        thread::Builder::new()
            .name("Bobby".into())
            .spawn_scoped(s, {
                let queue = queue.clone();
                || {
                    ServerThread::new(queue, 6535897).start();
                }
            })
            .unwrap();
    });
}
