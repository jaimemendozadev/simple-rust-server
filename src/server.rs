use std::io::Read;
use std::net::TcpListener;

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
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Running on {} ", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024].to_vec();
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:#?} ", String::from_utf8(buffer))
                        }

                        Err(e) => println!("Failed to read from connection: {} ", e),
                    }
                }
                Err(e) => print!("Failed to establish a connection: {} ", e),
            }
        }
    }
}
