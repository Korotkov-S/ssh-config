use clap::{Parser, Subcommand};
use inquire::Select;

mod password;
mod path;
mod ssh;
mod ssh_config;

/// The tool for working with a ssh config
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to ssh config file
    #[arg(short, long, default_value_t = path::get_default_path())]
    path: String,

    //password comands
    #[command(subcommand)]
    password: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Password {
        #[command(subcommand)]
        actions: PasswordActions,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum PasswordActions {
    Delete,
    Create,
}

fn main() {
    let args = Args::parse();

    let options = ssh_config::parse(args.path);

    let ssh_host = Select::new("Select a ssh host for connecting", options).prompt();

    match ssh_host {
        Ok(choice) => {
            if let Some(password) = args.password {
                match password {
                    Commands::Password { actions } => match actions {
                        PasswordActions::Create => {
                            password::new_password(choice);
                        }
                        PasswordActions::Delete => {
                            password::remove_password(choice);
                        }
                    },
                }
            } else {
                ssh::connect(choice);
            }
        }
        Err(_) => println!("Selected ssh host is not valid"),
    }
}
