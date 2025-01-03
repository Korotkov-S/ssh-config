use std::{
    fs::{create_dir_all, remove_file, File},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use inquire::Password;

use crate::{path::get_ssh_config_path, ssh_config::HostOption};

pub fn new_password(host_option: HostOption) {
    let status = Password::new("Enter password").prompt();

    match status {
        Ok(password) => {
            let password_path = get_password_path(get_ssh_config_path(), host_option.host_name);
            let password_file = create_password_file(password_path);

            match password_file {
                Ok(file) => {
                    let text = create_password_file_data(password);

                    let _ = write_password_file(file, text);
                }
                Err(error) => {
                    println!("Could not create new password file {error}")
                }
            }
        }
        Err(error) => {
            println!("Could not create new password file {error}")
        }
    }
}

pub fn remove_password(host_option: HostOption) {
    let password_path = get_password_path(get_ssh_config_path(), host_option.host_name);

    let _ = remove_password_file(password_path);
}

fn create_password_file_data(password: String) -> String {
    return format!("#!/usr/bin/env bash\necho \"{password}\"");
}

fn write_password_file(mut file: File, password: String) -> io::Result<()> {
    return file.write_all(password.as_bytes());
}

fn remove_password_file(password_path: PathBuf) -> io::Result<()> {
    return remove_file(password_path);
}

pub fn get_password_path(path: PathBuf, host_name: String) -> PathBuf {
    let mut password_path = path;
    password_path.push(host_name);
    password_path.set_extension("sh");

    return password_path;
}

fn create_password_file(password_path: PathBuf) -> io::Result<File> {
    let _ = create_dir_all(password_path.parent().as_ref().unwrap());

    let file = File::create_new(password_path.clone());

    let path = password_path.to_str();
    match path {
        Some(p) => {
            Command::new("chmod").arg("+x").arg(p).spawn().expect("msg");
        }
        None => {
            println!("NONE!");
        }
    }

    return file;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_password_file_data() -> Result<(), String> {
        assert_eq!(
            create_password_file_data(String::from("123456")),
            "#!/usr/bin/env bash\necho \"123456\""
        );
        Ok(())
    }

    #[test]
    fn test_get_password_path() -> Result<(), String> {
        let test_root_project_path = PathBuf::from(".");
        let password_path = get_password_path(test_root_project_path, String::from("file_name"));

        let expect = format!("./file_name.sh");

        match password_path.to_str() {
            Some(path) => {
                assert_eq!(path, expect.as_str());
            }
            None => {}
        }

        Ok(())
    }

    #[test]
    fn test_write_password_file_in_exist_folder() -> Result<(), String> {
        let test_root_project_path = PathBuf::from(".");
        let password_path = get_password_path(test_root_project_path, String::from("file_name"));

        let created_file = create_password_file(password_path.clone());

        match created_file {
            Ok(file) => {
                let result = write_password_file(file, String::from("TEST"));

                assert_eq!(result.is_ok(), true);
            }
            Err(_) => {}
        }

        // let deleted = remove_password_file(password_path.clone());
        // assert_eq!(deleted.is_ok(), true);

        Ok(())
    }

    #[test]
    fn test_write_password_file_with_non_exist_folder() -> Result<(), String> {
        let test_root_project_path = PathBuf::from("./test_ssh_config");

        let password_path = get_password_path(test_root_project_path, String::from("file_name"));

        let created_file = create_password_file(password_path.clone());

        match created_file {
            Ok(file) => {
                let result = write_password_file(file, String::from("TEST"));

                assert_eq!(result.is_ok(), true);
            }
            Err(_) => {}
        }

        let deleted = remove_password_file(password_path.clone());
        assert_eq!(deleted.is_ok(), true);

        Ok(())
    }
}
