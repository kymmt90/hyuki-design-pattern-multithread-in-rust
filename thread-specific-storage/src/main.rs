mod client_thread;
mod log;
mod ts_log;

use std::thread;

use client_thread::ClientThread;

fn main() {
    thread::scope(|s| {
        for name in ["Alice", "Bobby", "Chris"] {
            thread::Builder::new()
                .name(name.into())
                .spawn_scoped(s, || {
                    ClientThread::new(name.into()).run();
                })
                .unwrap();
        }
    });
}
