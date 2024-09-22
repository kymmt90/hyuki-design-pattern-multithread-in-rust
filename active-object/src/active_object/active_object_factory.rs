use std::{sync::Arc, thread};

use crate::active_object::{
    activation_queue::ActivationQueue, proxy::Proxy, scheduler_thread::SchedulerThread,
    servant::Servant, ArcActiveObject,
};

pub fn create_active_object() -> ArcActiveObject {
    let servant = Servant {};
    let scheduler = SchedulerThread::new(ActivationQueue::new());
    let proxy = Arc::new(Proxy::new(scheduler, servant));

    thread::spawn({
        let proxy = proxy.clone();
        move || {
            proxy.start_scheduler();
        }
    });

    proxy
}
