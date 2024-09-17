use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use crate::{request::Request, worker_thread::WorkerThread};

pub struct Channel {
    request_queue: Mutex<VecDeque<Request>>,
    takable: Condvar,
    puttable: Condvar,
    thread_pool: ThreadPool,
}

struct ThreadPool {
    threads: Vec<WorkerThread>,
}

impl Channel {
    const MAX_REQUEST: usize = 100;

    pub fn new(threads: usize) -> Self {
        Self {
            request_queue: Mutex::new(VecDeque::with_capacity(Self::MAX_REQUEST)),
            takable: Condvar::new(),
            puttable: Condvar::new(),
            thread_pool: ThreadPool::new(threads),
        }
    }

    pub fn start_workers(channel: Arc<Self>) {
        channel.thread_pool.start_workers(channel.clone());
    }

    pub fn put_request(&self, request: Request) {
        let mut queue = self.request_queue.lock().unwrap();
        while queue.len() >= Self::MAX_REQUEST {
            queue = self.puttable.wait(queue).unwrap();
        }

        queue.push_back(request);

        self.takable.notify_all();
    }

    pub fn take_request(&self) -> Request {
        let mut queue = self.request_queue.lock().unwrap();
        while queue.len() == 0 {
            queue = self.takable.wait(queue).unwrap();
        }

        let request = queue.pop_front().unwrap();

        self.puttable.notify_all();

        request
    }
}

impl ThreadPool {
    fn new(threads: usize) -> Self {
        let mut pool = Self {
            threads: Vec::with_capacity(threads),
        };

        let workers = (0..threads).map(|i| WorkerThread::new(format!("WorkerThread-{}", i)));
        pool.threads.extend(workers);

        pool
    }

    fn start_workers(&self, channel: Arc<Channel>) {
        for worker in &self.threads {
            let channel = channel.clone();
            worker.start(channel);
        }
    }
}

impl Iterator for ThreadPool {
    type Item = WorkerThread;

    fn next(&mut self) -> Option<Self::Item> {
        self.threads.pop()
    }
}
