use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use crate::active_object::method_request::MethodRequest;

pub struct ActivationQueue {
    request_queue: Mutex<VecDeque<Arc<dyn MethodRequest>>>,
    takable: Condvar,
    puttable: Condvar,
}

impl ActivationQueue {
    const MAX_METHOD_REQUESTS: usize = 100;

    pub fn new() -> Self {
        Self {
            request_queue: Mutex::new(VecDeque::new()),
            takable: Condvar::new(),
            puttable: Condvar::new(),
        }
    }

    pub fn put_request(&self, request: Arc<dyn MethodRequest>) {
        let mut queue = self.request_queue.lock().unwrap();
        while queue.len() >= Self::MAX_METHOD_REQUESTS {
            queue = self.puttable.wait(queue).unwrap();
        }

        queue.push_back(request);

        self.takable.notify_all();
    }

    pub fn take_request(&self) -> Arc<dyn MethodRequest> {
        let mut queue = self.request_queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.takable.wait(queue).unwrap();
        }

        let request = queue.pop_front().unwrap();

        self.puttable.notify_all();

        request
    }
}
