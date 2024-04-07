pub mod common;
pub mod ellipticcurve;
pub mod exponentiation;
use crate::zkp_auth::AuthenticationType;
use ellipticcurve::EllipticCurve;
use exponentiation::Exponentiation;
use num_bigint::BigUint;

pub trait Authenticate: Sync + Send {
    fn auth_type(&self) -> AuthenticationType;
    fn auth_id(&self) -> String;
    fn session_id(&self) -> String;
    fn get_random(&self) -> BigUint;
    fn registration(&self, nonce: &BigUint) -> (BigUint, BigUint);
    fn authentication(&self, secret: &BigUint) -> (BigUint, BigUint);
    fn challenge(&self) -> BigUint;
    fn response(&self, nonce: &BigUint, secret: &BigUint, challenge: &BigUint) -> BigUint;
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

pub fn get_authentication(authtype: AuthenticationType) -> Box<dyn Authenticate> {
    match authtype {
        AuthenticationType::EllipticCurve => return Box::new(EllipticCurve::new()),
        AuthenticationType::Exponentiation => return Box::new(Exponentiation::new()),
    }
}
