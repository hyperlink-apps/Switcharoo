use env_extract::{ConfigStruct, EnvVar};

#[derive(Clone, EnvVar)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

#[derive(ConfigStruct, Clone)]
pub struct Config {
    #[enumerated]
    pub environment: Environment,
    pub database_url: String,
    pub domain: String,
    pub secure: bool,
}
