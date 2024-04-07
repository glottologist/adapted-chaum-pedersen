use num_bigint::BigUint;
use rand::{thread_rng, Rng};

pub fn generate_random_string_of_length(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn get_random_int_within_bound(upper_bound: &BigUint) -> BigUint {
    let mut rng = thread_rng();
    let lower_bound = BigUint::from(1u32);
    rng.gen_range(lower_bound..upper_bound.clone())
}
