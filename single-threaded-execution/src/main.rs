mod gate;
mod user_thread;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use gate::Gate;
use user_thread::UserThread;

fn main() {
    println!("Testing Gate, hit CTRL+C to exit.");

    let gate = Arc::new(Mutex::new(Gate::new()));

    let people = [
        ("Alice", "Alaska"),
        ("Bobby", "Brazil"),
        ("Chris", "Canada"),
    ];

    thread::scope(|s| {
        for (name, address) in people {
            s.spawn({
                let gate = gate.clone();
                move || {
                    UserThread::new(gate, name, address).start();
                }
            });
        }
    })
}
