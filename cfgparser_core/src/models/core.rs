#[derive(serde::Deserialize, Debug)]
pub struct Configuration {
    pub host: String,
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
