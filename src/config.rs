use serde::{
    de::{Error, Expected},
    Deserialize, Serialize,
};
use serde_json::Result;

struct ExpectedLength<'a> {
    length: usize,
    expected_length: usize,
    context: &'a str,
}

impl<'a> Expected for ExpectedLength<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "expected {} elements but got {} in {}",
            self.length, self.expected_length, self.context
        )
    }
}

impl<'a> ExpectedLength<'a> {
    fn new(length: usize, expected_length: usize, context: &'a str) -> Self {
        ExpectedLength {
            context,
            expected_length,
            length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub name: String,
    pub fullscreen: bool,
    pub template: Template,
    #[serde(rename = "emailExampleDomain")]
    pub email_example_domain: String,
    #[serde(rename = "emailWhitelistedDomains")]
    pub email_whitelisted_domains: Vec<String>,
    #[serde(rename = "emailBlacklistedDomains")]
    pub email_blacklisted_domains: Vec<String>,
    #[serde(rename = "emailValidationFailedHelp")]
    pub email_validation_failed_help: String,
    #[serde(rename = "emailServerEndpoint")]
    pub email_server_endpoint: String,
    #[serde(rename = "emailMaxRecipients")]
    pub email_max_recipients: u32,
    #[serde(rename = "mirrorPreview")]
    pub mirror_preview: bool,
    #[serde(rename = "mirrorOutput")]
    pub mirror_output: bool,
}

impl Config {
    pub fn new(source: &str) -> Result<Config> {
        let config = serde_json::from_str::<Config>(source)?;
        if config.template.frames.len() < 1 {
            Err(serde_json::Error::invalid_length(
                0,
                &ExpectedLength::new(config.template.frames.len(), 1, "template.frames"),
            ))
        } else {
            Ok(config)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Template {
    pub width: f32,
    pub height: f32,
    pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Frame {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
