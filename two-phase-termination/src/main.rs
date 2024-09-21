use std::{sync::Arc, thread, time::Duration};

use countup_thread::CountupThread;

mod countup_thread;

fn main() {
    println!("main: BEGIN");

    let countup = Arc::new(CountupThread::new());
    let handle = thread::spawn({
        let countup = countup.clone();
        move || {
            countup.run();
        }
    });

    thread::sleep(Duration::from_secs(10));

    println!("main: shutdown_request");
    countup.shutdown_request();

    println!("main: join");
    handle.join().unwrap();

    println!("main: END");
}
