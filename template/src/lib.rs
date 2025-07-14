mod client;
mod error;
mod request;
mod route;
mod types;
mod methods;

pub use client::*;
pub use error::Error;
pub use request::Request;
pub use route::*;
pub use types::*;
pub use methods::*;

type Result<T> = std::result::Result<T, Error>;
