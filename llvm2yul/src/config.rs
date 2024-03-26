#[derive(Debug)]
pub struct Config {
    pub contract_name: String,

    pub entry: String,
}

impl Config {
    pub fn default(contract_name: String) -> Self {
        Self {
            contract_name,
            entry: "_entry".into(),
        }
    }
}
