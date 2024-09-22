use std::sync::Arc;

use crate::active_object::{
    method_request::{DisplayStringRequest, MakeStringRequest},
    result::{FutureResult, Result},
    scheduler_thread::SchedulerThread,
    servant::Servant,
    ActiveObject,
};

pub struct Proxy {
    scheduler: SchedulerThread,
    servant: Servant,
}

impl Proxy {
    pub fn new(scheduler: SchedulerThread, servant: Servant) -> Self {
        Self { scheduler, servant }
    }

    pub fn start_scheduler(&self) {
        self.scheduler.run();
    }
}

impl ActiveObject for Proxy {
    fn make_string(&self, count: usize, fillchar: char) -> Arc<dyn Result<String>> {
        let future = Arc::new(FutureResult::<String>::new());

        {
            let future = future.clone();
            let servant = self.servant.clone();

            self.scheduler.invoke(Arc::new(MakeStringRequest::new(
                servant, future, count, fillchar,
            )));
        }

        future
    }

    fn display_string(&self, string: &str) {
        self.scheduler.invoke(Arc::new(DisplayStringRequest::new(
            self.servant.clone(),
            string.into(),
        )));
    }
}
