pub mod authentication;
pub mod cli;
pub mod client;
pub mod errors;
pub mod server;
pub mod zkp_auth {
    // Dynamically include the Rust version of the protobuf schema generated at compile time.
    include!(concat!(env!("OUT_DIR"), "/zkp_auth.rs"));
}

use std::fmt;

impl fmt::Display for zkp_auth::AuthenticationType {
    // The `fmt` function is required by the `Display` trait. It defines how to format the value of `AuthenticationType`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let writable = match self {
            zkp_auth::AuthenticationType::Exponentiation => "Exponentiation",
            zkp_auth::AuthenticationType::EllipticCurve => "EllipticCurve",
        };
        // Write the string representation to the provided formatter. This is what will be output when the value is formatted with `{}`.
        write!(f, "{}", writable)
    }
}
