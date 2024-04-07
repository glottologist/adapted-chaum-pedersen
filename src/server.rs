use crate::authentication::Authenticate;
use crate::{
    authentication::get_authentication,
    zkp_auth::{
        auth_server::Auth, AuthTypeRequest, AuthTypeResponse, AuthenticationAnswerRequest,
        AuthenticationAnswerResponse, AuthenticationChallengeRequest,
        AuthenticationChallengeResponse, AuthenticationType, RegisterRequest, RegisterResponse,
    },
};
use moka::sync::Cache;
use num_bigint::BigUint;
use tonic::{Request, Response, Status};
use tracing::{debug, info};

#[derive(Clone, Debug)]
pub struct Registration {
    user: String,
    y1: BigUint,
    y2: BigUint,
}

#[derive(Clone, Debug)]
pub struct Challenge {
    user: String,
    r1: BigUint,
    r2: BigUint,
    c: BigUint,
}

pub struct ServerState {
    authenticator: Box<dyn Authenticate>,
    registrations: Cache<String, Registration>,
    challenges: Cache<String, Challenge>,
}

impl ServerState {
    pub fn new(use_ec: bool) -> Self {
        let auth = if use_ec {
            AuthenticationType::EllipticCurve
        } else {
            AuthenticationType::Exponentiation
        };
        info!("Using {} as authentication type", &auth);
        let authenticator = get_authentication(auth);
        Self {
            authenticator,
            registrations: Cache::builder().build(),
            challenges: Cache::builder().build(),
        }
    }
}

#[tonic::async_trait]
impl Auth for ServerState {
    async fn get_auth_type(
        &self,
        _request: Request<AuthTypeRequest>,
    ) -> Result<Response<AuthTypeResponse>, Status> {
        let auth = &self.authenticator.auth_type();
        Ok(Response::new(AuthTypeResponse { auth: *auth as i32 }))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received register request: {:?}", &inner_req);

        let reg = Registration {
            user: inner_req.user,
            y1: BigUint::from_bytes_be(&inner_req.y1),
            y2: BigUint::from_bytes_be(&inner_req.y2),
        };

        self.registrations.insert(reg.user.clone(), reg);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received challenge request: {:?}", &inner_req);

        let auth_id = self.authenticator.auth_id();
        let challenge = self.authenticator.challenge();
        let chal = Challenge {
            user: inner_req.user,
            r1: BigUint::from_bytes_be(&inner_req.r1),
            r2: BigUint::from_bytes_be(&inner_req.r2),
            c: challenge.clone(),
        };

        self.challenges.insert(auth_id.clone(), chal);

        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id,
            c: challenge.to_bytes_be(),
        }))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let inner_req = request.into_inner();

        debug!("Received challenge answer request: {:?}", &inner_req);

        let s = BigUint::from_bytes_be(&inner_req.s);

        let challenge = self
            .challenges
            .get(&inner_req.auth_id)
            .ok_or_else(|| Status::not_found("Unable to find prior challenge"))?;

        let registration = self
            .registrations
            .get(&challenge.user)
            .ok_or_else(|| Status::not_found("Unable to find prior registration"))?;

        let verified = self.authenticator.verify(
            &registration.y1,
            &registration.y2,
            &challenge.r1,
            &challenge.r2,
            &s,
            &challenge.c,
        );

        if verified {
            let session_id = self.authenticator.session_id();
            Ok(Response::new(AuthenticationAnswerResponse { session_id }))
        } else {
            Err(Status::unauthenticated("Unable to authenticate"))
        }
    }
}
