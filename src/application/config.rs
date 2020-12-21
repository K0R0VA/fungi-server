use std::env;
use dotenv::dotenv;

pub struct Configurator {
    server: String,
    postgre: String,
    mongo: String,
    secret_key: String
}

impl Default for  Configurator {
     fn default() -> Self {
        dotenv().ok();
        Configurator {
            server: env::var("CONFIGURATION_URL").expect("CONFIGURATION_URL is not set"),
            postgre: env::var("DATABASE_URL").expect("POSTGRE is not set"),
            mongo: env::var("MONGO_URL").expect("MONGO_URL is not set"),
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY is not set"),
        }
    }
}

impl Configurator {
    pub fn postgre(&self) -> &str {
        &self.postgre
    }
    pub fn mongo(&self) -> &str {
        &self.mongo
    }
    pub fn secret(&self) -> &str {
        &self.secret_key
    }
    pub fn server(&self) -> &str { &self.server }
}



