mod active_object;
mod display_client_thread;
mod maker_client_thread;

use std::thread;

use active_object::active_object_factory::create_active_object;
use display_client_thread::DisplayClientThread;
use maker_client_thread::MakerClientThread;

fn main() {
    let active_object = create_active_object();

    thread::scope(|s| {
        for name in ["Alice", "Bobby"] {
            thread::Builder::new()
                .name(name.into())
                .spawn_scoped(s, {
                    let active_object = active_object.clone();
                    move || {
                        MakerClientThread::new(active_object, name).run();
                    }
                })
                .unwrap();
        }

        thread::Builder::new()
            .name("Chris".into())
            .spawn_scoped(s, {
                let active_object = active_object.clone();
                move || {
                    DisplayClientThread::new(active_object).run();
                }
            })
            .unwrap();
    });
}
