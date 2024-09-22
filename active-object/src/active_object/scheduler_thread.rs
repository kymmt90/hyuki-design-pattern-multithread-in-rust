use std::sync::Arc;

use crate::active_object::{activation_queue::ActivationQueue, method_request::MethodRequest};

pub struct SchedulerThread {
    queue: ActivationQueue,
}

impl SchedulerThread {
    pub fn new(queue: ActivationQueue) -> Self {
        Self { queue }
    }

    pub fn invoke(&self, request: Arc<dyn MethodRequest>) {
        self.queue.put_request(request);
    }

    pub fn run(&self) {
        loop {
            let request = self.queue.take_request();

            request.execute();
        }
    }
}
