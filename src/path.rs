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
