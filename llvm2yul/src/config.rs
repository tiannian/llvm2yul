#[derive(Debug)]
pub struct Config {
    pub entry: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            entry: "_entry".into(),
        }
    }
}
