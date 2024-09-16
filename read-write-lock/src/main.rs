mod data;
mod reader_thread;
mod writer_thread;

use std::{sync::Arc, thread};

use data::Data;
use reader_thread::ReaderThread;
use writer_thread::WriterThread;

fn main() {
    let data = Arc::new(Data::new(10));

    thread::scope(|s| {
        for id in 0..6 {
            thread::Builder::new()
                .name(format!("Reader-{}", id))
                .spawn_scoped(s, {
                    let data = data.clone();
                    || {
                        ReaderThread::new(data).run();
                    }
                })
                .unwrap();
        }

        s.spawn({
            let data = data.clone();
            || {
                WriterThread::new(data, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into()).run();
            }
        });

        s.spawn({
            let data = data.clone();
            || {
                WriterThread::new(data, "abcdefghijklmnopqrstuvwxyz".into()).run();
            }
        });
    });
}
