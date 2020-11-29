// allows for for doing http::Method instead of http::method::Method
pub use request::Request;
pub use method::Method;
pub use request::ParseError;
pub use query_string::{QueryString, Value as QueryStringValue};

// make http folder visible for import
pub mod method;
pub mod request;
pub mod query_string;