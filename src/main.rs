use clap::Parser;
use inquire::Select;
use ssh2_config::{Host, ParseRule, SshConfig};
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::path::PathBuf;
use std::{fs::File, process::Command};

mod path {
    use dirs::home_dir;
    use std::path::PathBuf;

    pub fn get_default_path() -> String {
        let home = home_dir();
        let mut home_path = match home {
            Some(home) => home,
            None => PathBuf::new(),
        };

        home_path.push(".ssh/config");

        let path_str = match home_path.to_str() {
            Some(p) => String::from(p),
            None => String::from(""),
        };

        return path_str;
    }
}

/// The tool for working with a ssh config
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to ssh config file
    #[arg(short, long, default_value_t = path::get_default_path())]
    path: String,
}

struct HostOption {
    config: Host,
    value: String,
}

impl Display for HostOption {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let args = Args::parse();

    let config_path = args.path;

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

        let host_name = params.host_name.unwrap_or(String::from(""));

        let pattern = host.pattern.clone();
        let pattern_name = pattern[0].pattern.as_str();
        value.push_str(pattern_name);
        value.push_str(" - ");
        value.push_str(host_name.as_str());

        let own_host = HostOption {
            config: host.clone(),
            value: value.to_string(),
        };

        if !host_name.as_str().is_empty() {
            options.push(own_host);
        }
    }

    let ssh_host = Select::new("Select a ssh host for connecting", options).prompt();

    match ssh_host {
        Ok(choice) => {
            let mut host = String::from("");

            let pattern = choice.config.pattern.clone();
            let pattern_name = pattern[0].pattern.as_str();
            host.push_str(pattern_name);

            Command::new("ssh")
                .arg(host)
                .spawn()
                .unwrap()
                .wait()
                .expect("error ssh connections");
        }
        Err(_) => println!("Selected ssh host is not valid"),
    }
}
