use std::sync::Arc;

mod activation_queue;
pub mod active_object_factory;
mod method_request;
mod proxy;
mod result;
mod scheduler_thread;
mod servant;

pub trait ActiveObject: Send + Sync {
    fn make_string(&self, count: usize, fillchar: char) -> Arc<dyn result::Result<String>>;
    fn display_string(&self, string: &str);
}

pub type ArcActiveObject = Arc<dyn ActiveObject>;
