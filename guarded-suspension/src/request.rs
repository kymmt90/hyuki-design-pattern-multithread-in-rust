use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct RequestQueue {
    queue: Mutex<VecDeque<Request>>,
    not_empty: Condvar,
}

pub struct Request {
    name: String,
}

impl RequestQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
        }
    }

    pub fn get_request(&self) -> Request {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        queue.pop_front().unwrap()
    }

    pub fn put_request(&self, request: Request) {
        self.queue.lock().unwrap().push_back(request);
        self.not_empty.notify_all();
    }
}

impl Request {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ Request {} ]", self.name)
    }
}
