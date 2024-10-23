#[derive(Debug ,clone)]
pub struct Config {
    pub database_url :String
}

impl Config {
    pub fn init()->self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        Config {
            database_url:database_url
        }
    }
}
