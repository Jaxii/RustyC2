use config::FileFormat;
use config::{Config, File};
use serde::Deserialize;

/*
Reference:
https://blog.logrocket.com/configuration-management-in-rust-web-services/
*/

#[derive(Debug, Deserialize, Clone)]
pub struct FrontEndSettings {
    pub api_server: ApiServerSettings,
    pub bind_host: String,
    pub bind_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiServerSettings {
    pub host: String,
    pub port: u16,
}

impl FrontEndSettings {
    pub fn new() -> Self {
        let config_builder = Config::builder();
        let mut config: Self = Self {
            api_server: ApiServerSettings {
                host: String::from("127.0.0.1"),
                port: 9090,
            },
            bind_host: String::from("127.0.0.1"),
            bind_port: 8080,
        };

        match config_builder
            .add_source(File::new("config/default", FileFormat::Json))
            .build()
        {
            Ok(loaded_config) => {
                match loaded_config.try_deserialize() {
                    Ok(new_config) => config = new_config,
                    Err(_) => {}
                };
            }
            Err(_) => {}
        };

        return config;
    }
}
