pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        dotenvy::dotenv().ok();
        let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
        let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
        let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );
        let server_port_str = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let server_port: u16 = server_port_str
            .parse()
            .expect("SERVER_PORT must be a valid u16 port number");
        Config {
            database_url,
            server_port,
        }
    }
}
