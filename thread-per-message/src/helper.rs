use std::{thread, time::Duration};

pub fn handle(count: usize, c: char) {
    println!("      handle({}, {}) BEGIN", count, c);

    for _ in 0..count {
        thread::sleep(Duration::from_millis(100));
        print!("{}", c);
    }
    println!();

    println!("      handle({}, {}) END", count, c);
}
