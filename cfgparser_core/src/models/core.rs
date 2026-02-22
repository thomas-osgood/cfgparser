pub struct Configuration {
    pub host: Vec<u8>,
    pub port: i8,
}

impl Configuration {
    fn new(host: Vec<u8>, port: i8) -> Configuration {
        Configuration {
            host: host,
            port: port,
        }
    }
}
