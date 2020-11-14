use std::sync::Arc;
use argonautica::{Hasher, Verifier};

#[derive(Clone)]
pub struct CryptoService<'a> {
    pub key : Arc<&'a str>
}

impl<'a> CryptoService<'a> {
    pub fn hash_password(&self, password : String) -> String {
        Hasher::default()
            .with_secret_key(*self.key)
            .with_password(password)
            .hash()
            .expect("Error from hashing")
    }
    pub fn verify_password(&self, input_password: String, hash_password : &str) -> bool {
        Verifier::default()
            .with_secret_key(*self.key)
            .with_password(input_password)
            .with_hash(hash_password)
            .verify()
            .expect("Error from Verifier")
    }
}
