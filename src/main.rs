use clap::Parser;

mod toggle;
mod client;
mod config;
mod commands;

use commands::{Cli, Commands};


const CONFIG_NAME: &str = "toggle";


fn main() {
    let args = Cli::parse();
    let mut config = config::_load_config(CONFIG_NAME);
    let client = client::_create_client(&config);

    match args.command {
        Commands::Auth {api_key} => {
            match api_key {
                Some(key) => config.api_key = key,
                None => println!("Current auth: {}", config.api_key)
            }
        },
        Commands::Logout => config.api_key = String::from(""),

        Commands::Start { description} => toggle::start_entry(&client, &config, description),
        Commands::Stop => toggle::stop_entry(&client, &config),
        Commands::Current => match toggle::get_current_time_entry(&client, &config) {
            Some(entry) => println!("Current: {}", entry.description),
            None => println!("There is no current time entry")
        },

        Commands::Proxy { proxy_url } => {
            match proxy_url {
                Some(url) => config.proxy = url,
                None => println!("Current proxy: {}", config.proxy),
            }
        },
        Commands::Workspace { workspace_id } => {
            match workspace_id {
                Some(id) => config.workspace_id = id,
                None => println!("Current workspace: {}", config.workspace_id)
            }
        }

        Commands::Status => println!("{:?}", config)
    };

    config::_save_config(CONFIG_NAME, config);
}