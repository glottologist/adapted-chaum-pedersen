use crate::authentication::{get_authentication, Authenticate};
use crate::errors::{AuthenticationError, StatusAsError};
use crate::zkp_auth::auth_client::AuthClient;
use crate::zkp_auth::{
    AuthTypeRequest, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    AuthenticationType, RegisterRequest,
};
use num_bigint::BigUint; // For handling large integers in cryptographic operations
use rpassword::prompt_password; // To securely prompt for password input
use tonic::{transport::Channel, Request}; // Tonic for gRPC communication
use tracing::{debug, info}; // For logging

// Function to get the user's password securely, returns a BigUint representation
fn get_password() -> Result<BigUint, AuthenticationError> {
    match prompt_password("Enter password: ") {
        Ok(password) => Ok(BigUint::from_bytes_be(password.trim().as_bytes())),
        Err(_) => Err(AuthenticationError::CouldNotGetPassword),
    }
}

// Get the authentication type from the server so that the client can match the type of auth
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

// ClientRegistrar structure for handling user registration encapsulating the internal
// authenticator
pub struct ClientRegistrar {
    authenticator: Box<dyn Authenticate>,
}

impl ClientRegistrar {
    // Construct the ClientRegistrar, including requesting the auth type from the server
    pub async fn new(client: &mut AuthClient<Channel>) -> Result<Self, AuthenticationError> {
        let auth_type = get_auth_type(client).await?;
        let authenticator = get_authentication(auth_type);
        Ok(Self { authenticator })
    }

    // Register a user with the authentication server
    pub async fn register(
        &self,
        user: &str,
        client: &mut AuthClient<Channel>,
    ) -> Result<bool, AuthenticationError> {
        info!("Registering user '{}' with authentication server", user);

        let auth = &self.authenticator;
        let password = get_password()?; // Securely get the user's password from the terminal
        let (y1, y2) = auth.registration(&password); // Get the initial registration parameters
                                                     // based on the password
        let reg_request = RegisterRequest {
            user: user.to_string(),
            y1: y1.to_bytes_be(),
            y2: y2.to_bytes_be(),
        };

        debug!("Registering y1:{:?} and y2:{:?}", &y1, &y2);

        let _ = client
            .register(Request::new(reg_request))
            .await
            .map_err(|s| s.map_status_to_err())?; // Map the tonic error to a custom error
        Ok(true)
    }
}

// ClientAuthenticator structure for handling user authentication, encapsulating the internal
// authenticator and the nonce value k
pub struct ClientAuthenticator {
    pub authenticator: Box<dyn Authenticate>,
    pub k: BigUint,
}

impl ClientAuthenticator {
    // Construct the ClientAuthenticator, including requesting the auth type from the server
    pub async fn new(client: &mut AuthClient<Channel>) -> Result<Self, AuthenticationError> {
        let auth_type = get_auth_type(client).await?;
        let authenticator = get_authentication(auth_type);
        let k = authenticator.get_random(); // Generate the one time parameter k
        Ok(Self { authenticator, k })
    }

    // Authenticating a user with the server
    pub async fn authenticate(
        &self,
        user: &str,
        client: &mut AuthClient<Channel>,
    ) -> Result<bool, AuthenticationError> {
        info!("Authenticating user '{}' with authentication server", user);

        let auth = &self.authenticator;

        let password = get_password()?; // Securely get the user's password

        let (r1, r2) = auth.authentication(&self.k); // Generate authentication parameters

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

        info!("Authentication challenge received.");
        debug!("Received c {:?}", &c);

        let s = auth.response(&self.k, &password, &c); // Generate response to the challenge

        let answer_req = AuthenticationAnswerRequest {
            auth_id: challenge_response.auth_id,
            s: s.to_bytes_be(),
        };

        info!("Sending authentication challenge response.");

        debug!("Sent s {:?}", &s);

        let verify_response = client
            .verify_authentication(Request::new(answer_req))
            .await
            .map_err(|s| s.map_status_to_err())?; // Verify the challenge response with the server

        info!(
            "Session id received {:?}",
            &verify_response.into_inner().session_id
        );

        Ok(true)
    }
}
