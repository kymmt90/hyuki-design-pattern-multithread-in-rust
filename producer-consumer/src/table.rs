use std::{
    sync::{Condvar, Mutex},
    thread,
};

pub struct Table {
    buffer: Mutex<Buffer>,
    puttable: Condvar,
    takable: Condvar,
}

struct Buffer {
    buffer: Vec<String>,
    tail: usize,
    head: usize,
    count: usize,
}

impl Table {
    pub fn new(count: usize) -> Self {
        Self {
            buffer: Mutex::new(Buffer {
                buffer: vec!["".into(); count],
                tail: 0,
                head: 0,
                count: 0,
            }),
            puttable: Condvar::new(),
            takable: Condvar::new(),
        }
    }

    pub fn put(&self, cake: String) {
        let mut buffer = self.buffer.lock().unwrap();

        println!("{} puts {}", thread::current().name().unwrap(), cake);

        while buffer.count >= buffer.buffer.capacity() {
            buffer = self.puttable.wait(buffer).unwrap();
        }

        let tail = buffer.tail;
        buffer.buffer[tail] = cake;
        buffer.tail = (buffer.tail + 1) % buffer.buffer.capacity();
        buffer.count += 1;

        self.takable.notify_all();
    }

    pub fn take(&self) -> String {
        let mut buffer = self.buffer.lock().unwrap();

        while buffer.count == 0 {
            buffer = self.takable.wait(buffer).unwrap();
        }

        let head = buffer.head;
        let cake = buffer.buffer[head].clone();
        buffer.head = (buffer.head + 1) % buffer.buffer.capacity();
        buffer.count -= 1;

        self.puttable.notify_all();

        println!("{} takes {}", thread::current().name().unwrap(), cake);

        cake
    }
}
