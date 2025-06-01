use std::env;


pub struct Config {
    pub db_url: String
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DATABASE_URL").unwrap();
        Config {
            db_url
        }
    }
}