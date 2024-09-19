use std::{sync::Arc, thread};

use crate::data::{Data, FutureData, RealData};

pub fn request(count: i32, c: char) -> Arc<dyn Data> {
    println!("    request({}, {}) BEGIN", count, c);

    let future = Arc::new(FutureData::new());

    thread::spawn({
        let future = future.clone();
        move || {
            let realdata = RealData::new(count, c);
            future.set_real_data(realdata);
        }
    });

    println!("    request({}, {}) END", count, c);

    future
}
