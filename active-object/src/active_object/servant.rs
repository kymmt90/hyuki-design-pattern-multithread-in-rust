use std::{sync::Arc, thread, time::Duration};

use crate::active_object::{
    result::{RealResult, Result},
    ActiveObject,
};

#[derive(Clone)]
pub struct Servant {}

impl ActiveObject for Servant {
    fn make_string(&self, count: usize, fillchar: char) -> Arc<dyn Result<String>> {
        let mut buffer = Vec::with_capacity(count);

        for _ in 0..count {
            buffer.push(fillchar);

            thread::sleep(Duration::from_millis(100));
        }

        Arc::new(RealResult::new(buffer.into_iter().collect()))
    }

    fn display_string(&self, string: &str) {
        println!("display_string: {}", string);

        thread::sleep(Duration::from_millis(10));
    }
}
