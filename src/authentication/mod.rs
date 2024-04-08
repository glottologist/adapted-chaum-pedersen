pub mod common;
pub mod ellipticcurve;
pub mod exponentiation;
use crate::zkp_auth::AuthenticationType;
use ellipticcurve::EllipticCurve;
use exponentiation::Exponentiation;
use num_bigint::BigUint;

// Define a trait to encapsulate authentication behavior, ensuring it's compatible with asynchronous runtime
pub trait Authenticate: Sync + Send {
    // Return the type of authentication being used
    fn auth_type(&self) -> AuthenticationType;
    // Generate a unique identifier for the authentication session
    fn auth_id(&self) -> String;
    // Generate a unique session identifier to link challenge and verification steps
    fn session_id(&self) -> String;
    // Generate a random number
    fn get_random(&self) -> BigUint;
    // Process for registration, taking the secret and returning two values for the registration request
    fn registration(&self, secret: &BigUint) -> (BigUint, BigUint);
    // Process for authentication, taking a nonce and  returning two values for the authentication
    // request
    fn authentication(&self, nonce: &BigUint) -> (BigUint, BigUint);
    // Generate a challenge for the client, part of the authentication process
    fn challenge(&self) -> BigUint;
    // Generate a response to a challenge, using the nonce, secret, and challenge
    fn response(&self, nonce: &BigUint, secret: &BigUint, challenge: &BigUint) -> BigUint;
    // Verify the response to a challenge, confirming authenticity
    fn verify(
        &self,
        y1: &BigUint,
        y2: &BigUint,
        r1: &BigUint,
        r2: &BigUint,
        s: &BigUint,
        c: &BigUint,
    ) -> bool;
}

// Function to instantiate the appropriate authentication mechanism based on the given AuthenticationType
pub fn get_authentication(authtype: AuthenticationType) -> Box<dyn Authenticate> {
    match authtype {
        // Instantiate an EllipticCurve authenticator
        AuthenticationType::EllipticCurve => Box::new(EllipticCurve::new()),
        // Instantiate an Exponentiation authenticator
        AuthenticationType::Exponentiation => Box::new(Exponentiation::new()),
    }
}
