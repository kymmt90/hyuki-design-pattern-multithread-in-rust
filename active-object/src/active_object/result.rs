use std::sync::{Arc, Condvar, Mutex};

pub trait Result<T>: Send + Sync {
    fn get_result_value(&self) -> T;
}

pub struct FutureResult<T> {
    result: Mutex<Option<Arc<dyn Result<T>>>>,
    ready: Condvar,
}

impl<T> FutureResult<T> {
    pub fn new() -> Self {
        Self {
            result: Mutex::new(None),
            ready: Condvar::new(),
        }
    }

    pub fn set_result(&self, result: Arc<dyn Result<T>>) {
        let mut guard = self.result.lock().unwrap();
        *guard = Some(result);

        self.ready.notify_all();
    }
}

impl<T> Result<T> for FutureResult<T> {
    fn get_result_value(&self) -> T {
        let mut guard = self.result.lock().unwrap();
        while guard.is_none() {
            guard = self.ready.wait(guard).unwrap();
        }

        guard.as_ref().unwrap().get_result_value()
    }
}

pub struct RealResult<T> {
    value: T,
}

impl<T> RealResult<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone + Send + Sync> Result<T> for RealResult<T> {
    fn get_result_value(&self) -> T {
        self.value.clone()
    }
}
