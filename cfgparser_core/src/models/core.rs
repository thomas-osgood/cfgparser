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
        Configuration { host, port, scheme }
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

impl std::fmt::Display for SchemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemeType::HTTP => write!(f, "http"),
            SchemeType::HTTPS => write!(f, "https"),
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
