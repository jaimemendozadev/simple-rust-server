fn main() {
    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // Associated function
    // Self is alias for Struct itself
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Running on {} ", self.addr);
    }
}
