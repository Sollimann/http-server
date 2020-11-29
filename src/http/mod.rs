// allows for for doing http::Method instead of http::method::Method
pub use request::Request;
pub use method::Method;

// make http folder visible for import
pub mod method;
pub mod request;