use super::common::{generate_random_string_of_length, get_random_int_within_bound};
use super::Authenticate;
use crate::zkp_auth::AuthenticationType;
use num_bigint::BigUint;

#[derive(Debug)]
pub struct EllipticCurve {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub h: BigUint,
}

impl Authenticate for EllipticCurve {
    fn auth_type(&self) -> AuthenticationType {
        AuthenticationType::EllipticCurve
    }
    fn auth_id(&self) -> String {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn session_id(&self) -> String {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn get_random(&self) -> BigUint {
        unimplemented!("No support for Elliptic Curves yet")
    }

    fn registration(&self, _secret: &BigUint) -> (BigUint, BigUint) {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn challenge(&self) -> BigUint {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn response(&self, _nonce: &BigUint, _secret: &BigUint, _challenge: &BigUint) -> BigUint {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn authentication(&self, _nonce: &BigUint) -> (BigUint, BigUint) {
        unimplemented!("No support for Elliptic Curves yet")
    }
    fn verify(
        &self,
        y1: &BigUint,
        y2: &BigUint,
        r1: &BigUint,
        r2: &BigUint,
        s: &BigUint,
        c: &BigUint,
    ) -> bool {
        unimplemented!("No support for Elliptic Curves yet")
    }
}

impl EllipticCurve {
    pub fn new() -> Self {
        EllipticCurve {
            p: BigUint::from(0u32),
            q: BigUint::from(0u32),
            g: BigUint::from(0u32),
            h: BigUint::from(0u32),
        }
    }
}
