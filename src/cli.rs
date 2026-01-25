use crate::IconSet;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Notify {
        /// Application name for the notification
        #[arg(short, long, default_value_t = String::from("my_app"))]
        app_name: String,

        /// Replaces ID of the notification to replace
        #[arg(short = 'r', long, default_value_t = 0)]
        replaces_id: u32,

        /// Notification title or summary
        #[arg(short, long, default_value_t = String::from("A summary"))]
        title: String,

        /// Notification body text
        #[arg(short, long, default_value_t = String::from("Some body"))]
        body: String,

        /// Icon name
        #[arg(short, long, default_value_t = String::from("dialog-information"))]
        icon: String,

        /// Notification timeout in milliseconds
        #[arg(short = 's', long, default_value_t = 5000)]
        timeout: i32,
    },

    ListIcons {
        /// Which icon set to list
        #[arg(short, long, value_enum, default_value_t = IconSet::All)]
        set: IconSet,
    },
}
