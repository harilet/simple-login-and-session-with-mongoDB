use mongodb::bson::Document;
use rand::distributions::{Alphanumeric, DistString};
use sha2::{Digest, Sha512};

pub fn get_password_hash(password: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn get_random_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

pub fn compare_passwords(user_data: Document, password: String) -> bool {
    let mut raw_password = password;

    let secret_phrase = user_data
        .get_str("secret")
        .expect("Get secret from doc Error");
    raw_password.push_str(&secret_phrase);

    let password = get_password_hash(raw_password.to_string());
    let password_stored = user_data
        .get_str("password")
        .expect("Get password from doc Error");

    println!("{:?}:{:?}", password_stored, password);
    password_stored == password
}
