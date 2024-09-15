use std::fmt::Display;

#[derive(Debug)]
pub struct Gate {
    counter: i32,
    name: String,
    address: String,
}

impl Gate {
    pub fn new() -> Self {
        Self {
            counter: 0,
            name: "Nobody".into(),
            address: "Nowhere".into(),
        }
    }

    pub fn pass(&mut self, name: &str, address: &str) {
        self.counter += 1;
        self.name = name.into();
        self.address = address.into();
        self.check();
    }

    fn check(&self) {
        if self.name.chars().nth(0).unwrap() != self.address.chars().nth(0).unwrap() {
            println!("***** BROKEN ***** {self}");
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No. {}: {}, {}", self.counter, self.name, self.address)
    }
}
