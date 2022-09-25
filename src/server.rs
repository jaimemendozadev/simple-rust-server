// Every file in Rust is treated as a module
pub struct Server {
    addr: String,
}

impl Server {
    // Associated function
    // Self is alias for Struct itself
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Running on {} ", self.addr);
    }
}
