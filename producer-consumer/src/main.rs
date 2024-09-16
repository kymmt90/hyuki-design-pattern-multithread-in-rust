mod eater_thread;
mod maker_thread;
mod table;

use std::{sync::Arc, thread};

use table::Table;

fn main() {
    let table = Arc::new(Table::new(3));

    thread::scope(|s| {
        for (id, seed) in [("1", 31415), ("2", 92653), ("3", 58979)] {
            std::thread::Builder::new()
                .name(format!("MakerThread-{}", id))
                .spawn_scoped(s, {
                    let table = table.clone();
                    move || {
                        maker_thread::MakerThread::new(table, seed).run();
                    }
                })
                .unwrap();
        }

        for (id, seed) in [("1", 32384), ("2", 62643), ("3", 38327)] {
            std::thread::Builder::new()
                .name(format!("EaterThread-{}", id))
                .spawn_scoped(s, {
                    let table = table.clone();
                    move || {
                        eater_thread::EaterThread::new(table, seed).start();
                    }
                })
                .unwrap();
        }
    });
}
