#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub app_base_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("APPLICATION_URL").expect("APPLICATION_URL not set");
        let app_base_url =
            std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        Config {
            database_url,
            app_base_url,
        }
    }
}
