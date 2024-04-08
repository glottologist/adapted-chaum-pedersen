use num_bigint::BigUint;
use rand::{thread_rng, Rng};

// Define a function to generate a random alphanumeric string of a given length.
pub fn generate_random_string_of_length(size: usize) -> String {
    // Create a thread-local random number generator and generate an iterator of random alphanumeric characters,
    // take `size` number of characters, convert them to `char`, and collect into a `String`.
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric) // Use `Alphanumeric` distribution for alphanumeric chars.
        .take(size) // Take `size` elements from the iterator.
        .map(char::from) // Convert each `u8` to `char`.
        .collect() // Collect elements into a `String`.
}

// Define a function to generate a random `BigUint` within a specified upper bound.
pub fn get_random_int_within_bound(upper_bound: &BigUint) -> BigUint {
    let mut rng = thread_rng(); // Get a thread-local random number generator.
    let lower_bound = BigUint::from(1u32); // Define the lower bound as 1 to avoid generating 0.
                                           // Generate a random `BigUint` between `lower_bound` (inclusive) and `upper_bound` (exclusive).
    rng.gen_range(lower_bound..upper_bound.clone())
}
