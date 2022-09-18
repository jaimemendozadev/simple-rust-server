fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
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

    fn run(self) {}
}
