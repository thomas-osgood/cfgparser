#[cfg(test)]
mod unit_tests;

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: i64,
}

impl Configuration {
    pub fn new(host: String, port: i64) -> Configuration {
        Configuration {
            host: host,
            port: port,
        }
    }
}

/// default port for JSON deserialize if the field is not present.
fn default_port() -> i64 {
    80
}
