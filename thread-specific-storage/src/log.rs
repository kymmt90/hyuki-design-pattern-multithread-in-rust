use std::cell::RefCell;

use crate::ts_log::TsLog;

thread_local! {
    pub static TS_LOG: RefCell<TsLog> = RefCell::new(
        TsLog::new(
            format!("{}-log.txt", std::thread::current().name().unwrap())
        )
    );
}

pub fn println(s: &str) {
    TS_LOG.with_borrow_mut(|log| {
        log.println(s);
    });
}
