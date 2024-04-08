use acp::cli::{Cli, Command};
use acp::client::{ClientAuthenticator, ClientRegistrar};
use acp::server::ServerState;
use acp::zkp_auth::auth_client::AuthClient;
use acp::zkp_auth::auth_server::AuthServer;
use clap::Parser; // For command-line argument parsing
use tonic::transport::Channel; // For gRPC channel management
use tonic::transport::Server; // For gRPC server functionality
use tracing::{debug, error, info}; // For logging

// Asynchronously connect to the authentication server and return a gRPC client
async fn connect_to_server(server_address: &str) -> anyhow::Result<AuthClient<Channel>> {
    info!("Auth server address is {}", server_address);
    let target_with_scheme = format!("http://{}", server_address);
    let channel = Channel::from_shared(target_with_scheme)?.connect().await?;
    Ok(AuthClient::new(channel))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init(); // Initialize logging
    let cli = Cli::parse();

    // Match against the command specified by the user
    match cli.command {
        Command::Register(client_args) => {
            let mut client: AuthClient<Channel> =
                connect_to_server(&client_args.server_address.to_string()).await?; // Connect to the server

            let c = ClientRegistrar::new(&mut client).await?; // Create a new client registrar

            // Attempt to register the user
            match c.register(&client_args.user, &mut client).await? {
                true => info!("Successfully registered"),
                false => error!("Registration failed"),
            }
        }
        Command::Authenticate(client_args) => {
            let mut client: AuthClient<Channel> =
                connect_to_server(&client_args.server_address.to_string()).await?; // Connect to the server

            let c = ClientAuthenticator::new(&mut client).await?; // Create a new client authenticator

            // Attempt to authenticate the user
            match c.authenticate(&client_args.user, &mut client).await? {
                true => info!("Authentication successful"),
                false => error!("Authentication failed"),
            }
        }
        Command::Server(server_args) => {
            let binding_addr = format!("0.0.0.0:{}", server_args.port); // Determine the binding address
            let state = ServerState::new(server_args.use_elliptic_curve); // Initialize server state

            info!("Starting auth server on {}", binding_addr); // Log the server start
                                                               // Start the gRPC server and add the authentication service
            Server::builder()
                .add_service(AuthServer::new(state))
                .serve(binding_addr.parse()?) // Parse the address string into a SocketAddr
                .await?;
        }
    }
    Ok(())
}
