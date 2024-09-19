use std::{
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

pub trait Data {
    fn get_content(&self) -> String;
}

#[derive(Debug)]
pub struct FutureData {
    realdata: Mutex<Option<RealData>>,
    ready: Condvar,
}

impl FutureData {
    pub fn new() -> Self {
        Self {
            realdata: Mutex::new(None),
            ready: Condvar::new(),
        }
    }

    pub fn set_real_data(&self, realdata: RealData) {
        let mut lock = self.realdata.lock().unwrap();

        if lock.is_some() {
            return;
        }

        *lock = Some(realdata);

        self.ready.notify_all();
    }
}

impl Data for FutureData {
    fn get_content(&self) -> String {
        let mut lock = self.realdata.lock().unwrap();

        loop {
            if lock.is_none() {
                lock = self.ready.wait(lock).unwrap();
            } else {
                return lock.as_ref().unwrap().get_content();
            }
        }
    }
}

#[derive(Debug)]
pub struct RealData {
    content: String,
}

impl RealData {
    pub fn new(count: i32, c: char) -> Self {
        println!("        making RealData({}, {}) BEGIN", count, c);

        let mut buffer: Vec<char> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            buffer.push(c);
            thread::sleep(Duration::from_millis(100));
        }

        println!("        making RealData({}, {}) END", count, c);

        Self {
            content: buffer.into_iter().collect(),
        }
    }
}

impl Data for RealData {
    fn get_content(&self) -> String {
        self.content.clone()
    }
}
