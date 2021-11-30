pub struct Config {
    pub host: &'static str,
}

impl Config {
    pub fn new() -> Config{
        Config{
            host: ""
        }
    }
}
