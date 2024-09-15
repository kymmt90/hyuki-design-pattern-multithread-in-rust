use std::sync::{Arc, Mutex};

use crate::gate::Gate;

#[derive(Debug, Clone)]
pub struct UserThread {
    gate: Arc<Mutex<Gate>>,
    myname: String,
    myaddress: String,
}

impl UserThread {
    pub fn new(gate: Arc<Mutex<Gate>>, name: &str, address: &str) -> Self {
        Self {
            gate,
            myname: name.into(),
            myaddress: address.into(),
        }
    }

    pub fn start(&self) {
        println!("{}, BEGIN", self.myname);

        loop {
            self.gate
                .lock()
                .unwrap()
                .pass(&self.myname, &self.myaddress);
        }
    }
}
