use std::net::TcpListener;
use std::io::Read;
use crate::http::Request; // using 'crate' keyword means the root of your project
use std::convert::TryFrom;
use std::convert::TryInto;

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
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // array of 0 and of 1024 bytes
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                    println!("OK");
                },
                Err(e) => println!("Failed to estabish a connection: {}", e)
            }

            let res = listener.accept();
            if res.is_err(){
                continue;
            }
            let (stream, addr) = res.unwrap();
        }
    }
}
