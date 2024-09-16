use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Data {
    buffer: RwLock<Vec<char>>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: RwLock::new(vec!['*'; size]),
        }
    }

    pub fn read(&self) -> Vec<char> {
        let lock = self.buffer.read().unwrap();

        self.do_read(&lock)
    }

    fn do_read(&self, buffer: &RwLockReadGuard<Vec<char>>) -> Vec<char> {
        let mut newbuf: Vec<char> = Vec::with_capacity(buffer.len());
        for i in 0..buffer.len() {
            newbuf.push(buffer[i]);
            self.slowly();
        }

        newbuf
    }

    pub fn write(&self, c: char) {
        let mut buffer = self.buffer.write().unwrap();
        self.do_write(&mut buffer, c);
    }

    fn do_write(&self, buffer: &mut RwLockWriteGuard<Vec<char>>, c: char) {
        for i in 0..buffer.len() {
            buffer[i] = c;
            self.slowly();
        }
    }

    fn slowly(&self) {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
