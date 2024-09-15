pub struct Person {
    name: String,
    address: String,
}

impl Person {
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ Person: name = {}, address = {} ]",
            self.name, self.address
        )
    }
}
