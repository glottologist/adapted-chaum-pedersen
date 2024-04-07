use super::common::{generate_random_string_of_length, get_random_int_within_bound};
use super::Authenticate;
use crate::zkp_auth::AuthenticationType;
use num_bigint::{BigInt, BigUint};
use num_traits::One;

#[derive(Debug)]
pub struct Exponentiation {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub h: BigUint,
}

impl Authenticate for Exponentiation {
    fn auth_type(&self) -> AuthenticationType {
        AuthenticationType::Exponentiation
    }
    fn auth_id(&self) -> String {
        generate_random_string_of_length(50)
    }
    fn session_id(&self) -> String {
        generate_random_string_of_length(100)
    }
    fn get_random(&self) -> BigUint {
        get_random_int_within_bound(&self.q)
    }

    fn registration(&self, nonce: &BigUint) -> (BigUint, BigUint) {
        let y1 = self.g.modpow(&nonce, &self.p);
        let y2 = self.h.modpow(&nonce, &self.p);
        (y1, y2)
    }
    fn challenge(&self) -> BigUint {
        get_random_int_within_bound(&self.q)
    }
    fn response(&self, nonce: &BigUint, secret: &BigUint, challenge: &BigUint) -> BigUint {
        let cs = BigInt::from(challenge * secret);
        let n = BigInt::from(nonce.clone());
        let q = BigInt::from(self.q.clone());
        let s = (&n - &cs).modpow(&BigInt::one(), &q);
        s.to_biguint().unwrap() //   hh_or(BigUint::from(0u32))
    }
    fn authentication(&self, secret: &BigUint) -> (BigUint, BigUint) {
        let r1 = self.g.modpow(&secret, &self.p);
        let r2 = self.h.modpow(&secret, &self.p);
        (r1, r2)
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
        let ver1 = (self.g.modpow(s, &self.p) * y1.modpow(c, &self.p)) % &self.p;
        let ver2 = (self.h.modpow(s, &self.p) * y2.modpow(c, &self.p)) % &self.p;
        ver1 == *r1 && ver2 == *r2
    }
}

impl Exponentiation {
    pub fn new() -> Self {
        Exponentiation {
            p: BigUint::from(10009u32),
            q: BigUint::from(5004u32),
            g: BigUint::from(3u32),
            h: BigUint::from(2892u32),
        }
    }
}

mod tests {
    use {super::*, proptest::prelude::*};

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

    fn password_as_biguint_strategy() -> impl Strategy<Value = BigUint> {
        (1_0u32..=2_0) // Ensuring at least 4 digits, up to the max u32 value
            .prop_map(|n| n as usize) // Convert the number to a string
            .prop_map(|n| generate_random_string_of_length(n)) // Convert the number to a string
            .prop_map(|s| BigUint::from_bytes_be(s.trim().as_bytes())) // Convert the number to a string
    }
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

            prop_assert!(&auth, "Authentication should have been succesful: secret:{}, y1:{}, y2:{}, r1:{}, r2:{}, k:{}, s:{}, c:{}", &secret,&y1,&y2,&r1,&r2,&k,&s,&c);
        }


    }
}
