use std::env;

pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment varialble not set");
        Config { db_url }
    }
}
