use std::sync::Arc;
use argonautica::{Hasher, Verifier};
use futures::compat::Future01CompatExt;

#[derive(Clone)]
pub struct CryptoService<'a> {
    pub key : Arc<&'a str>
}

impl<'a> CryptoService<'a> {
    pub async fn hash_password(&self, password : &str) -> String {
        Hasher::default()
            .with_secret_key(*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
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
