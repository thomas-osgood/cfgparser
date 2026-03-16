#[cfg(test)]
mod unit_tests;

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemeType {
    HTTP,
    HTTPS,
}

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_scheme")]
    pub scheme: SchemeType,
}

impl Configuration {
    pub fn new(host: String, port: i64, scheme: SchemeType) -> Configuration {
        Configuration {
            host: host,
            port: port,
            scheme: scheme,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            host: "localhost".to_string(),
            port: 80,
            scheme: SchemeType::default(),
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

/// default scheme for JSON deserialize if the field is not present.
fn default_scheme() -> SchemeType {
    SchemeType::default()
}
