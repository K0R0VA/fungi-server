use std::sync::Arc;
use argonautica::{Hasher, Verifier};
use futures::compat::Future01CompatExt;

#[derive(Clone)]
pub struct Descriptor {
    key: Arc<String>
}

impl Descriptor {
    pub fn new(key: String) -> Self {
        Descriptor {
            key : Arc::new(key)
        }
    }
}

impl Descriptor {
    pub async fn hash_password(&self, password : String) -> String {
        Hasher::default()
            .with_secret_key((*self.key).clone())
            .configure_hash_len(16)
            .configure_iterations(48)
            .configure_memory_size(1024)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .expect("Hashing panic")
    }
    pub async fn verify_password(&self, input_password: String, hash_password : &str) -> bool {
        Verifier::default()
            .with_secret_key(&*self.key)
            .with_password(input_password)
            .with_hash(hash_password)
            .verify_non_blocking()
            .compat()
            .await
            .expect("Error from Verifier")
    }
}
