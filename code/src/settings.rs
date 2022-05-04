use config::{Config, ConfigError, File};
use serde::Deserialize;
use config::FileFormat;

/*
Reference:
https://blog.logrocket.com/configuration-management-in-rust-web-services/
*/

#[derive(Debug, Deserialize, Clone)]
pub struct Listener
{
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings
{
    pub listener: Listener,
}

/*
Reference:
https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
*/
impl Settings
{
    pub fn new() -> Result<Self, ConfigError>
    {
        let s = Config::builder()
            .add_source(File::new("config/default", FileFormat::Json))
            .build()?;

        s.try_deserialize()
    }
}
