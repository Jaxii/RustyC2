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
    pub client: ClientSettings,
    pub listener: Listener,
    pub implant: Implant
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientSettings
{
    pub main_tag: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener
{
    pub http: HttpListenerSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpListenerSettings
{
    pub address: String,
    pub port: u16,
    pub pull_method: String,
    pub pull_endpoint: String,
    pub push_method: String,
    pub push_endpoint: String,
    pub default_page_path: String,
    pub default_error_page_path: String,
    pub auth_cookie_regex: String,
    pub responses: HttpResponsesSettingsGroup
}

#[derive(Debug, Deserialize, Clone)]
pub struct Implant
{
    pub sleep: u32,
    pub tasks: ImplantTaskSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImplantTaskSettings
{
    pub use_commands_codes: bool,
    pub use_alt_names: bool,
    pub commands: Vec<ImplantTaskCommand>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImplantTaskCommand
{
    pub name: String,
    pub description: String,
    pub code: String,
    pub alt_name: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpResponsesSettings
{
    pub status_code: u16,
    pub status_code_reason: String,
    pub http_version: String,
    pub headers: Vec<HttpHeader>
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpResponsesSettingsGroup
{
    pub default_success: HttpResponsesSettings,
    pub default_error: HttpResponsesSettings,
    pub implant_success: HttpResponsesSettings,
    pub implant_error: HttpResponsesSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpHeader
{
    pub name: String,
    pub value: String
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
