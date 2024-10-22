use config::{self, Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub db_user: String,
    pub password: String,
    pub db_port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.password, self.host, self.db_port, self.db_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.db_user, self.password, self.host, self.db_port
        )
    }
}

pub fn get_configuration() -> Result<DatabaseSettings, config::ConfigError> {
    dotenv().ok();
    let db_settings = match get_db_settings() {
        Ok(settings) => settings,
        Err(e) => return Err(e),
    };
    Ok(db_settings)
}

pub fn get_db_settings() -> Result<DatabaseSettings, config::ConfigError> {
    let mut dbsettings = Config::default();
    dbsettings.merge(Environment::new())?;
    dbsettings.try_into()
}
