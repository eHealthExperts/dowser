use super::CONFIG;

#[derive(Default, Clone)]
pub struct Config {
    pub vpn_endpoint_ip: String,
    pub source_port: i32,
    pub verbose: bool,
    pub use_nat_t: bool,
}

pub struct ConfigBuilder {
    config: Config,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn with_nat_t() -> Config {
        let mut new_cfg = CONFIG.clone();
        new_cfg.use_nat_t = true;
        new_cfg
    }
}

impl ConfigBuilder {
    fn new() -> ConfigBuilder {
        let config = Config {
            ..Default::default()
        };
        ConfigBuilder { config }
    }

    pub fn verbose(&mut self) -> &mut ConfigBuilder {
        self.config.verbose = true;
        self
    }

    pub fn source_port(&mut self, port: &str) -> &mut ConfigBuilder {
        let parsed_port = match port.parse::<i32>() {
            Ok(p) => p,
            _ => panic!("Unable to parse port argument"),
        };
        self.config.source_port = parsed_port;
        self
    }

    pub fn vpn_endpoint_ip(&mut self, ip: &str) -> &mut ConfigBuilder {
        // TODO syntactically validate IP here
        self.config.vpn_endpoint_ip = ip.to_owned();
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
