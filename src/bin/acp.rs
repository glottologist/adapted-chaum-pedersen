use acp::cli::{Cli, Command};
use acp::client::{ClientAuthenticator, ClientRegistrar};
use acp::server::ServerState;
use acp::zkp_auth::auth_client::AuthClient;
use acp::zkp_auth::auth_server::AuthServer;
use clap::Parser;
use tonic::transport::Channel;
use tonic::transport::Server;
use tracing::{debug, error, info};

async fn connect_to_server(server_address: &str) -> anyhow::Result<AuthClient<Channel>> {
    info!("Auth server address is {}", server_address);
    let target_with_scheme = format!("http://{}", server_address);
    let channel = Channel::from_shared(target_with_scheme)?.connect().await?;
    Ok(AuthClient::new(channel))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Register(client_args) => {
            let mut client: AuthClient<Channel> =
                connect_to_server(&client_args.server_address.to_string()).await?;

            let c = ClientRegistrar::new(&mut client).await?;

            match c.register(&client_args.user, &mut client).await? {
                true => info!("Succesfully registered"),
                false => error!("Registration failed"),
            }
        }
        Command::Authenticate(client_args) => {
            let mut client: AuthClient<Channel> =
                connect_to_server(&client_args.server_address.to_string()).await?;

            let c = ClientAuthenticator::new(&mut client).await?;

            match c.authenticate(&client_args.user, &mut client).await? {
                true => info!("Authentication succesful"),
                false => error!("Authentication failed"),
            }
        }
        Command::Server(server_args) => {
            let binding_addr = format!("0.0.0.0:{}", server_args.port);
            let state = ServerState::new(server_args.use_elliptic_curve);

            info!("Starting auth server on {}", binding_addr);
            Server::builder()
                .add_service(AuthServer::new(state))
                .serve(binding_addr.parse()?)
                .await?;
        }
    }
    Ok(())
}
