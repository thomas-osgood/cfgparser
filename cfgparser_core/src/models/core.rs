#[cfg(test)]
mod unit_tests;

pub enum SchemeType {
    HTTP,
    HTTPS,
}

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

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            host: "localhost".to_string(),
            port: 80,
        }
    }
}

impl Default for SchemeType {
    fn default() -> Self {
        SchemeType::HTTP
    }
}

impl ToString for SchemeType {
    fn to_string(&self) -> String {
        match *self {
            SchemeType::HTTP => "http".to_string(),
            SchemeType::HTTPS => "https".to_string(),
        }
    }
}

/// default port for JSON deserialize if the field is not present.
fn default_port() -> i64 {
    80
}
