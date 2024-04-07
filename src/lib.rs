pub mod authentication;
pub mod cli;
pub mod client;
pub mod errors;
pub mod server;
pub mod zkp_auth {
    include!(concat!(env!("OUT_DIR"), "/zkp_auth.rs"));
}

use std::fmt;

impl fmt::Display for zkp_auth::AuthenticationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let writable = match self {
            zkp_auth::AuthenticationType::Exponentiation => "Exponentiation",
            zkp_auth::AuthenticationType::EllipticCurve => "EllipticCurve",
        };
        write!(f, "{}", writable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
