use crate::authentication::get_authentication;
use crate::authentication::Authenticate;
use crate::errors::{AuthenticationError, StatusAsError};
use crate::zkp_auth::auth_client::AuthClient;
use crate::zkp_auth::{
    AuthTypeRequest, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    AuthenticationType, RegisterRequest,
};
use num_bigint::BigUint;
use rpassword::prompt_password;
use tonic::{transport::Channel, Request};
use tracing::{debug, info};

fn get_password() -> Result<BigUint, AuthenticationError> {
    match prompt_password("Enter password: ") {
        Ok(password) => Ok(BigUint::from_bytes_be(password.trim().as_bytes())),
        Err(_) => Err(AuthenticationError::CouldNotGetPassword),
    }
}

async fn get_auth_type(
    client: &mut AuthClient<Channel>,
) -> Result<AuthenticationType, AuthenticationError> {
    let response = client
        .get_auth_type(Request::new(AuthTypeRequest {}))
        .await
        .map_err(|_| AuthenticationError::UnableToGetAuthTypeFromServer)?
        .into_inner();
    let e = AuthenticationType::from_i32(response.auth)
        .ok_or_else(|| AuthenticationError::UnableToGetAuthTypeFromServer)?;
    Ok(e)
}

pub struct ClientRegistrar {
    authenticator: Box<dyn Authenticate>,
}

impl ClientRegistrar {
    pub async fn new(client: &mut AuthClient<Channel>) -> Result<Self, AuthenticationError> {
        let auth_type = get_auth_type(client).await?;
        let authenticator = get_authentication(auth_type);
        Ok(Self { authenticator })
    }

    pub async fn register(
        &self,
        user: &str,
        client: &mut AuthClient<Channel>,
    ) -> Result<bool, AuthenticationError> {
        info!("Registering user '{}' with authentication server", user);

        let auth = &self.authenticator;
        let password = get_password()?;
        let (y1, y2) = auth.registration(&password);
        let reg_request = RegisterRequest {
            user: user.to_string(),
            y1: y1.to_bytes_be(),
            y2: y2.to_bytes_be(),
        };

        debug!("Registering y1:{:?} and y2:{:?}", &y1, &y2);

        let _ = client
            .register(Request::new(reg_request))
            .await
            .map_err(|s| s.map_status_to_err())?;
        Ok(true)
    }
}
pub struct ClientAuthenticator {
    pub authenticator: Box<dyn Authenticate>,
    pub k: BigUint,
}

impl ClientAuthenticator {
    pub async fn new(client: &mut AuthClient<Channel>) -> Result<Self, AuthenticationError> {
        let auth_type = get_auth_type(client).await?;
        let authenticator = get_authentication(auth_type);
        let k = authenticator.get_random();
        Ok(Self { authenticator, k })
    }

    pub async fn authenticate(
        &self,
        user: &str,
        client: &mut AuthClient<Channel>,
    ) -> Result<bool, AuthenticationError> {
        info!("Authenticating user '{}' with authentication server", user);

        let auth = &self.authenticator;

        let password = get_password()?;

        let (r1, r2) = auth.authentication(&self.k);

        let challenge_req = AuthenticationChallengeRequest {
            user: user.to_string(),
            r1: r1.to_bytes_be(),
            r2: r2.to_bytes_be(),
        };

        debug!("Authenticating r1:{:?} and r2:{:?}", &r1, &r2);

        let challenge_response = client
            .create_authentication_challenge(Request::new(challenge_req))
            .await
            .map_err(|s| s.map_status_to_err())?
            .into_inner();

        let c = BigUint::from_bytes_be(&challenge_response.c);

        info!("Authentication challenge recieved.");
        debug!(" Received c {:?}", &c);

        let s = auth.response(&self.k, &password, &c);

        let answer_req = AuthenticationAnswerRequest {
            auth_id: challenge_response.auth_id,
            s: s.to_bytes_be(),
        };

        info!("Sending authentication challenge response.");

        debug!("Sent s {:?}", &s);

        let verify_response = client
            .verify_authentication(Request::new(answer_req))
            .await
            .map_err(|s| s.map_status_to_err())?;

        info!(
            "Session id received {:?}",
            &verify_response.into_inner().session_id
        );

        Ok(true)
    }
}
