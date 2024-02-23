use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command()]
#[clap(about="CLI Toggle wrapper", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Authorize with API key
    Auth {
        api_key: Option<String>
    },
    /// Delete store API key
    Logout,
    /// Set proxy url to send requests via it
    Proxy {
        /// Optional proxy url to set
        proxy_url: Option<String>
    },
    Workspace {
        workspace_id: Option<u32>
    },

    /// Start the time entry
    Start {
        /// Description for time entry
        description: String,
    },
    /// Stop the time entry
    Stop,
    /// Get current time entry
    Current,

    /// Get current config
    Status,
}