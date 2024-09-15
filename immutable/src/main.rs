mod person;
mod print_person_thread;

use std::{sync::Arc, thread};

use person::Person;
use print_person_thread::PrintPersonThread;

fn main() {
    let person = Arc::new(Person::new("Alice", "Alaska"));

    thread::scope(|s| {
        for name in ["1", "2", "3"] {
            thread::Builder::new()
                .name(name.into())
                .spawn_scoped(s, {
                    let person = person.clone();
                    move || {
                        PrintPersonThread::new(person).run();
                    }
                })
                .unwrap();
        }
    });
}
