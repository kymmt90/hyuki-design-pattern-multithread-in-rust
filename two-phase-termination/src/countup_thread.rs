use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    thread,
    time::Duration,
};

pub struct CountupThread {
    is_shutdown_requested: AtomicBool,
    countup: Mutex<Countup>,
}

impl CountupThread {
    pub fn new() -> CountupThread {
        Self {
            is_shutdown_requested: AtomicBool::new(false),
            countup: Mutex::new(Countup::new()),
        }
    }

    pub fn shutdown_request(&self) {
        self.is_shutdown_requested.store(true, Ordering::SeqCst);
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.is_shutdown_requested.load(Ordering::SeqCst)
    }

    pub fn run(&self) {
        while !self.is_shutdown_requested() {
            self.countup.lock().unwrap().do_work();
        }

        self.do_shutdown();
    }

    fn do_shutdown(&self) {
        println!(
            "do_shutdown: counter = {}",
            self.countup.lock().unwrap().counter
        );
    }
}

struct Countup {
    counter: u64,
}

impl Countup {
    fn new() -> Countup {
        Self { counter: 0 }
    }

    fn do_work(&mut self) {
        self.counter += 1;

        println!("do_work: counter = {}", self.counter);

        thread::sleep(Duration::from_millis(500));
    }
}
