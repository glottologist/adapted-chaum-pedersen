pub mod cli;
pub mod errors;
pub mod server;
pub mod zkp_auth {
    include!(concat!(env!("OUT_DIR"), "/zkp_auth.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;
}
