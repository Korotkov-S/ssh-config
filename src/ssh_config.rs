use ssh2_config::{Host, ParseRule, SshConfig};
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;

pub struct HostOption {
    pub config: Host,
    pub value: String,
    pub host_name: String,
}

impl Display for HostOption {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

pub fn parse(config_path: String) -> Vec<HostOption> {
    let path = PathBuf::from(config_path);

    let mut reader =
        io::BufReader::new(File::open(path).expect("Could not open configuration file"));

    let config = SshConfig::default()
        .parse(&mut reader, ParseRule::STRICT)
        .expect("Failed to parse configuration");

    let hosts = config.get_hosts();

    let mut options: Vec<HostOption> = Vec::new();

    for host in hosts {
        let params = host.params.clone();
        let mut value = String::new();

        let host_name = params.host_name.unwrap_or_default();

        let pattern = host.pattern.clone();
        let pattern_name = pattern[0].pattern.as_str();
        value.push_str(pattern_name);
        value.push_str(" - ");
        value.push_str(host_name.as_str());

        if !host_name.is_empty() {
            let own_host = HostOption {
                value: value.to_string(),
                config: host.clone(),
                host_name,
            };

            options.push(own_host);
        }
    }

    return options;
}
