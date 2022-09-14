use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn get_random_short_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
