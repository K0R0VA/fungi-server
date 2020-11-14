use std::env;
use dotenv::dotenv;
use std::sync::Arc;

use crate::middleware::crypto_service::CryptoService;

#[derive(Clone)]
pub struct Config {
    db_addr : String,
    server_addr : String,
    secret_key : String
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let db_addr = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let server_addr = env::var("CONFIGURATION_URL").expect("CONFIGURATION_URL is not set");
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY is not set");
        Config {
            db_addr,
            server_addr,
            secret_key
        }
    }
    pub fn server(&self) -> &str {
        &*self.server_addr
    }
    pub fn database(&self) -> &str {
        &*self.db_addr
    }
    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key : Arc::new(&*self.secret_key)
        }
    }
}