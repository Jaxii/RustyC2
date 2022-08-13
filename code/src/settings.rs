use config::{Config, ConfigError, File};
use serde::Deserialize;
use config::FileFormat;

/*
Reference:
https://blog.logrocket.com/configuration-management-in-rust-web-services/
*/

#[derive(Debug, Deserialize, Clone)]
pub struct Settings
{
    pub listener: Listener,
    pub implant: Implant
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener
{
    pub port: u16,
    pub address: String,
    pub http: HttpListenerSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpListenerSettings
{
    pub pull_method: String,
    pub pull_endpoint: String,
    pub push_method: String,
    pub push_endpoint: String,
    pub cookie_name: String,
    pub default_page_path: String,
    pub default_error_page_path: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Implant
{
    pub sleep: u32,
}

/*
Reference:
https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
*/
impl Settings
{
    pub fn new() -> Result<Self, ConfigError>
    {
        let s: Config = Config::builder()
            .add_source(File::new("config/default", FileFormat::Json))
            .build()?;

        s.try_deserialize()
    }
}
