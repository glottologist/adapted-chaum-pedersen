use clap::{Args, Parser, Subcommand};
use std::{
    io::{Error, ErrorKind, Result},
    net::{SocketAddr, ToSocketAddrs},
};

// Function to resolve a network target (e.g., "localhost:8080") into a SocketAddr
fn resolve_target(target: &str) -> Result<SocketAddr> {
    // Attempt to convert the target string into an iterator of SocketAddr
    let socketaddr = target.to_socket_addrs()?.next().ok_or_else(|| {
        // If no addresses are found, return an AddrNotAvailable error
        Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Could not find destination {target}"),
        )
    })?;
    Ok(socketaddr)
}

// CLI structure definition using clap for command-line argument parsing
#[derive(Parser)]
#[clap(
    author, // Automatically populated with the crate authors from Cargo.toml
    version, // Automatically populated with the crate version from Cargo.toml
    about = "An adapted Chaum-Pedersen protocol", // Short description
    long_about = "A gRPC implementation of the Chaum-Pedersen protocol, adapted for 1 factor authentication" // Longer description
)]
pub struct Cli {
    // Define the command structure for the CLI, supporting subcommands
    #[command(subcommand)]
    pub command: Command,
}

// Define arguments for the client-related commands
#[derive(Args)]
pub struct ClientArgs {
    // The server address, parsed by the resolve_target function to ensure validity
    #[arg(short, long, value_parser = resolve_target, help = "The address of the authentication server")]
    pub server_address: SocketAddr,

    // User ID for authentication, required for client commands
    #[arg(short, long = "user", help = "The user id for authentication")]
    pub user: String,
}

// Define arguments for the server command
#[derive(Args)]
pub struct ServerArgs {
    // The port number on which the server should listen, parsed as an unsigned 32-bit integer
    #[arg(
        short,
        long,
        help = "The port on which to bind the authentication server"
    )]
    pub port: u32,
    // Flag to indicate whether elliptic curve cryptography should be used instead of exponentiation
    #[arg(
        short = 'e',
        long = "use-elliptic-curve",
        help = "Indicates if the client/server pair should use elliptic curves rather than exponents."
    )]
    pub use_elliptic_curve: bool,
}

// Enum to represent the possible CLI commands, each associated with its specific arguments
#[derive(Subcommand)]
pub enum Command {
    #[command(aliases = ["r"])]
    Register(ClientArgs),
    #[command(aliases = ["a"])]
    Authenticate(ClientArgs),
    #[command(aliases = ["s"])]
    Server(ServerArgs),
}

mod tests {
    use {
        super::*,
        proptest::{
            prelude::{Just, Strategy}, // For creating custom test strategies
            prop_oneof,
            proptest,
        },
        test_case::test_case, // For parameterized tests
    };

    // Test cases for the resolve_target function with expected success
    #[test_case("127.0.0.1:1024"; "when url is loopback")]
    #[test_case("localhost:1024"; "when url is localhost")]
    #[test_case("localhost:0"; "when url has 0 port")]
    fn test_resolve_target(url: &str) {
        let target = resolve_target(url);
        assert!(
            target.is_ok(),
            "Expected the target to be resolved successfully."
        );
    }

    // Test cases for the resolve_target function with expected failures
    #[test_case("localhost:65536"; "when url port is higher than maximum port")]
    fn test_resolve_target_failures(url: &str) {
        let target = resolve_target(url);
        assert!(
            target.is_err(),
            "Expected the target resolution to fail due to an invalid port."
        );
    }

    // Strategy for generating invalid domain names
    fn invalid_domain() -> impl Strategy<Value = String> {
        let scheme = prop_oneof![Just("http://"), Just("https://")];
        let www = prop_oneof![Just("www."), Just("")];
        let domain = "[a-z]{5,10}";
        let suffix = prop_oneof![
            Just(".com"),
            Just(".net"),
            Just(".io"),
            Just(".xyz"),
            Just(".co.uk")
        ];

        // Combine the parts into a full domain name
        (scheme, www, domain, suffix).prop_map(|(scheme, www, domain, suffix)| {
            format!("{}{}{}{}", scheme, www, domain, suffix)
        })
    }

    // Strategy for generating valid port numbers
    fn valid_port_strategy() -> impl Strategy<Value = u32> {
        prop_oneof![1024u32..65535u32,] // Range of valid port numbers
    }

    // Property-based test for the resolve_target function with artificially constructed domains
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))] // Configure the number of cases
        #[test]
        fn test_resolve_target_with_good_ports_prop(domain in invalid_domain(), port in valid_port_strategy()) {
            let url = format!("{}:{}", domain, port); // Combine domain and port into a URL
            let target = resolve_target(&url); // Attempt to resolve the URL
            assert!(target.is_err(), "Expected an error when resolving artificially constructed domain: {}", url);
        }
    }
}
