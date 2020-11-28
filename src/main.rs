fn main() {
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// Struct definition
struct Server {
    addr: String,
}

// Struct implementation
impl Server {
    // associated function
    // same as a staticmethod in Python
    fn new(addr: String) -> Self {
        // 'Self' is alias for 'Server'
        Self {
            addr
        }
    }

    // method
    // always takes a 'self'
    fn run(self) {

    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: String
}

enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}