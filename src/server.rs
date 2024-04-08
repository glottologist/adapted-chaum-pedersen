use crate::{
    authentication::{get_authentication, Authenticate},
    zkp_auth::{
        auth_server::Auth, AuthTypeRequest, AuthTypeResponse, AuthenticationAnswerRequest,
        AuthenticationAnswerResponse, AuthenticationChallengeRequest,
        AuthenticationChallengeResponse, AuthenticationType, RegisterRequest, RegisterResponse,
    },
};
use moka::sync::Cache;
use num_bigint::BigUint;
use tonic::{Request, Response, Status}; // Tonic library for gRPC support
use tracing::{debug, info}; // Tracing library for logging

// Struct representing user registration data including initial setup parameters
#[derive(Clone, Debug)]
pub struct Registration {
    user: String,
    y1: BigUint,
    y2: BigUint,
}

// Struct representing a challenge issued for authentication and the returned challenge
#[derive(Clone, Debug)]
pub struct Challenge {
    user: String,
    r1: BigUint,
    r2: BigUint,
    c: BigUint,
}

// Server's state including authenticator, user registrations, and challenges
pub struct ServerState {
    authenticator: Box<dyn Authenticate>, // Authentication logic encapsulation
    registrations: Cache<String, Registration>, // Cache for user registrations
    challenges: Cache<String, Challenge>, // Cache for authentication challenges
}

impl ServerState {
    // Constructor for ServerState including choosing the type of authentication
    pub fn new(use_ec: bool) -> Self {
        let auth = if use_ec {
            AuthenticationType::EllipticCurve
        } else {
            AuthenticationType::Exponentiation
        };

        info!("Using {} as authentication type", &auth);

        let authenticator = get_authentication(auth); // Get the authenticator based on the chosen method
        Self {
            authenticator,
            registrations: Cache::builder().build(),
            challenges: Cache::builder().build(),
        }
    }
}

// Implementing asynchronous trait for handling authentication  gRPC calls
#[tonic::async_trait]
impl Auth for ServerState {
    // Method to get the type of authentication being used by the server.  This wasn't in the
    // initial protobuf spec but thought it was a nicer way to handle more than one auth type
    async fn get_auth_type(
        &self,
        _request: Request<AuthTypeRequest>,
    ) -> Result<Response<AuthTypeResponse>, Status> {
        let auth = &self.authenticator.auth_type(); // Get authentication type from the authenticator currently being used.
        Ok(Response::new(AuthTypeResponse { auth: *auth as i32 }))
    }

    // Registering a the user setup parameters to be used later for authentication
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received register request: {:?}", &inner_req);

        // Build the registration type to be stored for the user
        let reg = Registration {
            user: inner_req.user,
            y1: BigUint::from_bytes_be(&inner_req.y1),
            y2: BigUint::from_bytes_be(&inner_req.y2),
        };

        // Insert the registration into the cache
        self.registrations.insert(reg.user.clone(), reg);

        Ok(Response::new(RegisterResponse {}))
    }

    // Create a challenge for user authentication
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received challenge request: {:?}", &inner_req);

        let auth_id = self.authenticator.auth_id(); // Generate an authentication ID
        let challenge = self.authenticator.challenge(); // Generate a challenge value
                                                        //
                                                        // Build the challenge type to be stored for the user
        let chal = Challenge {
            user: inner_req.user,
            r1: BigUint::from_bytes_be(&inner_req.r1),
            r2: BigUint::from_bytes_be(&inner_req.r2),
            c: challenge.clone(),
        };

        // Insert the challenge into the cache
        self.challenges.insert(auth_id.clone(), chal);

        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id,
            c: challenge.to_bytes_be(), // Return the challenge as bytes
        }))
    }

    // Verify the response to an authentication challenge
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received challenge answer request: {:?}", &inner_req);

        let s = BigUint::from_bytes_be(&inner_req.s);

        //In order to verify, we need the challenge and registration for the user from the cache
        let challenge = self
            .challenges
            .get(&inner_req.auth_id)
            .ok_or_else(|| Status::not_found("Unable to find prior challenge"))?;

        let registration = self
            .registrations
            .get(&challenge.user)
            .ok_or_else(|| Status::not_found("Unable to find prior registration"))?;

        // Verify the user authentication
        let verified = self.authenticator.verify(
            &registration.y1,
            &registration.y2,
            &challenge.r1,
            &challenge.r2,
            &s,
            &challenge.c,
        );

        if verified {
            let session_id = self.authenticator.session_id(); // Generate a session ID for the authenticated session
            Ok(Response::new(AuthenticationAnswerResponse { session_id }))
        } else {
            Err(Status::unauthenticated("Unable to authenticate"))
        }
    }
}
