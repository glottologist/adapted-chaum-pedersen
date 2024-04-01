use acp::cli::{Cli, Command};
use acp::server::ServerState;
use acp::zkp_auth::auth_server::AuthServer;
use clap::Parser;
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Client(_client) => {}
        Command::Server(server) => {
            let binding_addr = format!("0.0.0.0:{}", server.port);
            let state = ServerState::new(server.use_ec);

            info!("Starting auth server on {}", binding_addr);
            Server::builder()
                .add_service(AuthServer::new(state))
                .serve(binding_addr.parse()?)
                .await?;
        }
    }
    Ok(())
}
