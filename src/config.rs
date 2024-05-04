use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub name: String,
    pub fullscreen: bool,
    pub template: Template,
}

impl Config {
    pub fn new(source: &str) -> Result<Config> {
        serde_json::from_str(source)
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
