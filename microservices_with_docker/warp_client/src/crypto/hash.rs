use argon2::{
    Config,
    hash_encoded,
    verify_encoded,
};
use rand::Rng;

pub fn hash_with_argon2(credential: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    hash_encoded(credential, &salt, &config).unwrap()
}

pub fn verify_with_argon2(hash: &str, credential: &[u8]) -> bool {
    verify_encoded(hash, credential).unwrap_or(false)
}