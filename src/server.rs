use std::net::TcpListener;

// Struct definition
pub struct Server {
    addr: String,
}

// Struct implementation
impl Server {
    // associated function
    // same as a staticmethod in Python
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // method
    // always takes a 'self'
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        loop {
            let listener = TcpListener::bind(&self.addr).unwrap();
        }
    }
}
