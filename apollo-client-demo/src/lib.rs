use config::{Config, ConfigError, File};
use lazy_static::*;
use serde::{self, Deserialize, Serialize};
use apollo_client::ClientConfig;

lazy_static! {
  #[allow(deprecated)]
  static ref SETTINGS: Settings = Settings::new().unwrap();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub client_config: ClientConfig<String, Vec<String>>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        // 为了复用一个 config 目录
        s.merge(File::with_name("./apollo-client-demo/config/default.yaml"))?;
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}

pub fn get_settings() -> &'static Settings {
    &SETTINGS
}

pub fn get_apollo_config() -> &'static ClientConfig<String, Vec<String>> {
    &SETTINGS.client_config
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_file_exist() {
        let file = File::with_name("config/default.yaml");
        dbg!(file);
    }

    #[test]
    fn test_get_apollo_config() {
        //assert_eq!(get_proxy_config().host_port, "0.0.0.0:5432");
        dbg!(get_apollo_config());
    }

    #[test]
    fn test_get_settings() {
        //assert_eq!(get_settings().credentials.ssl.enable, false);
        dbg!(get_settings());
    }
}
