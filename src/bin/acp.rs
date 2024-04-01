use acp::cli::{Cli, Command};
use clap::Parser;
use std::{
    io::{Error, ErrorKind, Result},
    net::{SocketAddr, ToSocketAddrs},
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Client(client) => {}
        Command::Server(server) => {}
    }
    Ok(())
}
