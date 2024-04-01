pub mod cli;
pub mod errors;
pub mod acp {
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/acp.messages.rs"));
    }
}

use acp::messages;
#[cfg(test)]
mod tests {
    use super::*;
}
