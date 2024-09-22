use std::sync::Arc;

use crate::active_object::{result::FutureResult, servant::Servant, ActiveObject as _};

pub trait MethodRequest: Send + Sync {
    fn execute(&self);
}

pub struct MakeStringRequest {
    servant: Servant,
    future: Arc<FutureResult<String>>,
    count: usize,
    fillchar: char,
}

impl MakeStringRequest {
    pub fn new(
        servant: Servant,
        future: Arc<FutureResult<String>>,
        count: usize,
        fillchar: char,
    ) -> Self {
        Self {
            servant,
            future,
            count,
            fillchar,
        }
    }
}

impl MethodRequest for MakeStringRequest {
    fn execute(&self) {
        let result = self.servant.make_string(self.count, self.fillchar);
        self.future.set_result(result);
    }
}

pub struct DisplayStringRequest {
    servant: Servant,
    string: String,
}

impl DisplayStringRequest {
    pub fn new(servant: Servant, string: String) -> Self {
        Self { servant, string }
    }
}

impl MethodRequest for DisplayStringRequest {
    fn execute(&self) {
        self.servant.display_string(&self.string);
    }
}
