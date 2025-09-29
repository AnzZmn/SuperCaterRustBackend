use serde::Deserialize;


#[derive(Deserialize, Clone)]
pub struct Config{
      pub database: DataBaseConfig,
      pub application : ApplicationConfig,
}


#[derive(Deserialize, Clone)]

pub  struct DataBaseConfig{
      pub url: String,
}

impl DataBaseConfig {
    pub fn connection_string(&self) -> String {
        self.url.clone()
    }
}

#[derive(Deserialize, Clone)]

pub struct ApplicationConfig{
      pub host: String,
      pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        cfg.try_deserialize()
    }
}
