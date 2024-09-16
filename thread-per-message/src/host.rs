use std::thread::{self, JoinHandle};

use crate::helper;

pub fn request(count: usize, c: char) -> JoinHandle<()> {
    println!("  request({}, {}) BEGIN", count, c);

    let handle = thread::spawn(move || {
        helper::handle(count, c);
    });

    println!("  request({}, {}) END", count, c);

    handle
}
