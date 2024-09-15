mod changer_thread;
mod data;
mod saver_thread;

use std::{sync::Arc, thread};

use changer_thread::ChangerThread;
use data::Data;
use saver_thread::SaverThread;

fn main() {
    let data = Arc::new(Data::new("data.txt", "(empty)"));

    thread::scope(|s| {
        thread::Builder::new()
            .name("ChangerThread".into())
            .spawn_scoped(s, {
                let data = data.clone();
                || {
                    ChangerThread::new(data).run();
                }
            })
            .unwrap();

        thread::Builder::new()
            .name("SaverThread".into())
            .spawn_scoped(s, {
                let data = data.clone();
                || {
                    SaverThread::new(data).run();
                }
            })
            .unwrap();
    });
}
