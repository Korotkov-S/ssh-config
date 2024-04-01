use clap::Parser;
use inquire::Select;
use ssh2_config::{Host, ParseRule, SshConfig};
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::{fs::File, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
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

    let mut reader =
        io::BufReader::new(File::open(config_path).expect("Could not open configuration file"));

    let config = SshConfig::default()
        .parse(&mut reader, ParseRule::STRICT)
        .expect("Failed to parse configuration");

    dbg!(&config.get_hosts()[1].params.user);

    let hosts = config.get_hosts();

    let mut options: Vec<HostOption> = Vec::new();

    for host in hosts {
        match &host.params.host_name {
            Some(host_name) => {
                let own_host = HostOption {
                    config: host.clone(),
                    value: host_name.clone(),
                };
                options.push(own_host);
            }
            None => println!("Host name not found"),
        }
    }

    let ssh_host = Select::new("Select a ssh host for connecting", options).prompt();

    match ssh_host {
        Ok(choice) => {
            println!("ssh root@{}", choice.value);

            let mut host = String::from("");
            host.push_str(choice.config.params.user.expect("user not found").as_str());
            host.push_str("@");
            host.push_str(choice.value.as_str());

            println!("ssh -- {}", host.as_str());

            Command::new("ssh")
                .arg(host)
                .spawn()
                .unwrap()
                .wait()
                .expect("00");
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
