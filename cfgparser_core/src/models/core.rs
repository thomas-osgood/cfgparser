#[derive(serde::Deserialize, Debug)]
pub struct Configuration {
    pub host: String,
    pub port: i8,
}

impl Configuration {
    pub fn new(host: String, port: i8) -> Configuration {
        Configuration {
            host: host,
            port: port,
        }
    }
}
