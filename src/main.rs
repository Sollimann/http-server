// import files
mod server;
mod http;

// from file
use server::Server;
use http::Request;
use http::Method;

fn main() {

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
