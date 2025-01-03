use std::process::Command;

use crate::{password::get_password_path, path::get_ssh_config_path, ssh_config::HostOption};

pub fn connect(host_option: &HostOption) {
    let mut host = String::from("");

    let pattern = host_option.config.pattern.clone();
    let pattern_name = pattern[0].pattern.as_str();
    host.push_str(pattern_name);

    let mut ssh = Command::new("ssh");

    let password_path = get_password_path(get_ssh_config_path(), host_option.host_name.as_str());
    if let Some(path) = password_path.to_str() {
        if password_path.exists() {
            ssh.env("SSH_ASKPASS", path)
                .env("SSH_ASKPASS_REQUIRE", "force");
        }
    }

    ssh.arg(host)
        .spawn()
        .unwrap()
        .wait()
        .expect("error ssh connections");

    ssh.env_remove("SSH_ASKPASS");
}
