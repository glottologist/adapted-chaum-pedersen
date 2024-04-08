use super::common::{generate_random_string_of_length, get_random_int_within_bound};
use super::Authenticate;
use crate::zkp_auth::AuthenticationType;
use num_bigint::{BigInt, BigUint};
use num_traits::One;

// Define a structure `Exponentiation` for the exponentiation-based authentication mechanism.
#[derive(Debug)]
pub struct Exponentiation {
    pub p: BigUint, // Prime number, part of the public key.
    pub q: BigUint, // Prime number, a divisor of p-1, part of the private key.
    pub g: BigUint, // Generator of the cyclic group.
    pub h: BigUint, // Another generator of the cyclic group, used in the cryptographic scheme.
}

// Implement the `Authenticate` trait for the `Exponentiation` struct.
impl Authenticate for Exponentiation {
    // Return the authentication type, indicating this uses exponentiation-based authentication.
    fn auth_type(&self) -> AuthenticationType {
        AuthenticationType::Exponentiation
    }

    // Generate a random identifier for an authentication session.
    fn auth_id(&self) -> String {
        generate_random_string_of_length(50)
    }

    // Generate a unique identifier for a session after successful authentication.
    fn session_id(&self) -> String {
        generate_random_string_of_length(100)
    }

    // Get a random `BigUint` within the range of `1` to `q`.
    fn get_random(&self) -> BigUint {
        get_random_int_within_bound(&self.q)
    }

    // Registration function that calculates `y1` and `y2` based on a given `secret`.
    fn registration(&self, secret: &BigUint) -> (BigUint, BigUint) {
        let y1 = self.g.modpow(&secret, &self.p);
        let y2 = self.h.modpow(&secret, &self.p);
        (y1, y2)
    }

    // Generate a random challenge for the authentication process.
    fn challenge(&self) -> BigUint {
        get_random_int_within_bound(&self.q)
    }

    // Calculate the response to a challenge during authentication.
    fn response(&self, nonce: &BigUint, secret: &BigUint, challenge: &BigUint) -> BigUint {
        let cs = BigInt::from(challenge * secret);
        let n = BigInt::from(nonce.clone());
        let q = BigInt::from(self.q.clone());
        let s = (&n - &cs).modpow(&BigInt::one(), &q);
        s.to_biguint().unwrap()
    }

    // Registration function that calculates `r1` and `r2` based on a given `nonce`.
    fn authentication(&self, nonce: &BigUint) -> (BigUint, BigUint) {
        let r1 = self.g.modpow(&nonce, &self.p);
        let r2 = self.h.modpow(&nonce, &self.p);
        (r1, r2)
    }

    // Verify the authentication attempt using the generated and received values.
    fn verify(
        &self,
        y1: &BigUint,
        y2: &BigUint,
        r1: &BigUint,
        r2: &BigUint,
        s: &BigUint,
        c: &BigUint,
    ) -> bool {
        let ver1 = (self.g.modpow(s, &self.p) * y1.modpow(c, &self.p)) % &self.p;
        let ver2 = (self.h.modpow(s, &self.p) * y2.modpow(c, &self.p)) % &self.p;
        ver1 == *r1 && ver2 == *r2
    }
}

impl Exponentiation {
    pub fn new() -> Self {
        // Create the Exponentiation with definied initial parameters
        Exponentiation {
            p: BigUint::from(10009u32),
            q: BigUint::from(5004u32),
            g: BigUint::from(3u32),
            h: BigUint::from(2892u32),
        }
    }
}

// Unit and property-based tests for the `Exponentiation` authentication mechanism.
mod tests {
    use {super::*, proptest::prelude::*};

    // Test to ensure `g` is a valid generator of the cyclic group.
    #[test]
    fn g_should_be_a_generator_of_prime_order() {
        let e = Exponentiation::new();
        let should_be_one = e.g.modpow(&e.q, &e.p);
        assert_eq!(
            should_be_one,
            BigUint::from(1u32),
            "g is not a generator of the group"
        );
    }

    // Similar test for `h`, ensuring it's also a valid generator.
    #[test]
    fn h_should_be_a_generator_of_prime_order() {
        let e = Exponentiation::new();
        let should_be_one = e.h.modpow(&e.q, &e.p);
        assert_eq!(
            should_be_one,
            BigUint::from(1u32),
            "h is not a generator of the group"
        );
    }

    // Define a strategy for generating random `BigUint` values for testing.
    fn password_as_biguint_strategy() -> impl Strategy<Value = BigUint> {
        (1_0u32..=2_0)
            .prop_map(|n| n as usize)
            .prop_map(|n| generate_random_string_of_length(n))
            .prop_map(|s| BigUint::from_bytes_be(s.trim().as_bytes()))
    }

    // Property-based test to verify the full authentication process.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn test_full_authentication(secret in password_as_biguint_strategy()) {
            let e = Exponentiation::new();
            let (y1, y2) = e.registration(&secret);
            let  k = e.get_random();
            let (r1,r2) = e.authentication(&k);
            let c = e.challenge();
            let s = e.response(&k,&secret,&c);
            let auth = e.verify(&y1,&y2,&r1,&r2,&s,&c);

            prop_assert!(&auth, "Authentication should have been successful: secret:{}, y1:{}, y2:{}, r1:{}, r2:{}, k:{}, s:{}, c:{}", &secret,&y1,&y2,&r1,&r2,&k,&s,&c);
        }
    }
}
