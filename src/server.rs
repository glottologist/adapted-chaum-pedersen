use crate::zkp_auth::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
};
use moka::sync::Cache;
use tonic::{Request, Response, Status};

use tracing::{error, info};
pub struct ServerState {
    use_ec: bool,
    registrations: Cache<String, RegisterRequest>,
}

impl ServerState {
    pub fn new(use_ec: bool) -> Self {
        Self {
            use_ec,
            registrations: Cache::builder().build(),
        }
    }
}

#[tonic::async_trait]
impl Auth for ServerState {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id: "0".to_string(),
            c: 0,
        }))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        Ok(Response::new(AuthenticationAnswerResponse {
            session_id: "0".to_string(),
        }))
    }
}
