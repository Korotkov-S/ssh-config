use dirs::home_dir;
use std::path::PathBuf;

pub fn get_default_path() -> String {
    let mut home = get_home_dir();

    home.push(".ssh/config");

    let path_str = match home.to_str() {
        Some(p) => String::from(p),
        None => String::from(""),
    };

    return path_str;
}

pub fn get_home_dir() -> PathBuf {
    let home = home_dir();
    return match home {
        Some(home) => home,
        None => PathBuf::new(),
    };
}

pub fn get_ssh_config_path() -> PathBuf {
    let home = home_dir();
    let mut home_path = match home {
        Some(home) => home,
        None => PathBuf::new(),
    };

    home_path.push(".ssh_config");

    return home_path;
}
