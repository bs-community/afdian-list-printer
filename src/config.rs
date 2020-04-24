use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub account: String,
    pub password: String,
    pub amount: f32,
    pub target_files: Vec<String>,
}
