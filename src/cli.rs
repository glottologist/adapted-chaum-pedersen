use clap::{ArgAction, Args, Parser, Subcommand};
use std::{
    io::{Error, ErrorKind, Result},
    net::{SocketAddr, ToSocketAddrs},
};

fn resolve_target(target: &str) -> Result<SocketAddr> {
    let socketaddr = target.to_socket_addrs()?.next().ok_or_else(|| {
        Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Could not find destination {target}"),
        )
    })?;
    Ok(socketaddr)
}

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "An adapted Chaum-Pedersen protocol",
    long_about = "A gRPC implementation of the Chaum-Pedersen protocol, adapted for 1 factor authentication"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct ClientArgs {
    #[arg(short, long, value_parser = resolve_target, help = "The address of the authentication server")]
    pub server_address: SocketAddr,

    // Flag indicating whether a secure connection should be established, parsed as a boolean value.
    #[arg(short, long = "user", help = "The user id for authentication")]
    pub user: String,

    #[arg(action = ArgAction::SetFalse, short='e', long = "use-ec", help = "Indicates if the client/server pair should use elliptic curves rather than exponents.")]
    pub use_ec: bool,
}

#[derive(Args)]
pub struct ServerArgs {
    #[arg(
        short,
        long,
        help = "The port on which to bind the authentication server"
    )]
    pub port: u32,
    #[arg(action = ArgAction::SetFalse, short='e' ,long = "use-ec", help = "Indicates if the client/server pair should use elliptic curves rather than exponents.")]
    pub use_ec: bool,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(aliases = ["c"])]
    Client(ClientArgs),
    #[command(aliases = ["s"])]
    Server(ServerArgs),
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        proptest::{
            prelude::{Just, ProptestConfig, Strategy},
            prop_oneof, proptest,
        },
        test_case::test_case,
    };

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

    #[test_case("localhost:65536"; "when url port is higher than maximum port")]
    fn test_resolve_target_failures(url: &str) {
        let target = resolve_target(url);
        assert!(
            target.is_err(),
            "Expected the target resolution to fail due to an invalid port."
        );
    }

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

        (scheme, www, domain, suffix).prop_map(|(scheme, www, domain, suffix)| {
            format!("{}{}{}{}", scheme, www, domain, suffix)
        })
    }

    fn valid_port_strategy() -> impl Strategy<Value = u32> {
        prop_oneof![1024u32..65535u32,]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn test_resolve_target_with_good_ports_prop(domain in invalid_domain(), port in valid_port_strategy()) {
            let url = format!("{}:{}", domain, port);
            let target = resolve_target(&url);
            assert!(target.is_err(), "Expected an error when resolving artificially constructed domain: {}", url);
        }
    }
}
