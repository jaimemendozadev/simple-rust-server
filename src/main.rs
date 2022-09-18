fn main() {
    let get: Method = Method::GET;
    let post: Method = Method::POST;
    let delete: Method = Method::DELETE;
    let put: Method = Method::PUT;

    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run();
}

struct Server {
    addr: String,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
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
