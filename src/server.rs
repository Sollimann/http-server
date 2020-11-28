// Struct definition
pub struct Server {
    addr: String,
}

// Struct implementation
impl Server {
    // associated function
    // same as a staticmethod in Python
    pub fn new(addr: String) -> Self {
        // 'Self' is alias for 'Server'
        Self {
            addr
        }
    }

    // method
    // always takes a 'self'
    pub fn run(self) {

    }
}
