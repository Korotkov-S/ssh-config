use dirs::home_dir;
use std::path::PathBuf;

pub fn get_default_path() -> String {
    let mut home = get_home_dir();

    home.push(".ssh/config");

    return String::from(home.to_str().unwrap_or_default());
}

pub fn get_home_dir() -> PathBuf {
    return home_dir().unwrap_or_default();
}

pub fn get_ssh_config_path() -> PathBuf {
    let home = home_dir();
    let mut home_path = home.unwrap_or_default();

    home_path.push(".ssh_config");

    return home_path;
}
