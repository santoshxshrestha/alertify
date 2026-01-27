use clap::Parser;
use std::error::Error;

pub mod actions;
pub mod cli;
pub mod icons;
pub mod notification;
use cli::Cli;
use cli::Commands;

use icons::utils::handle_icon_listing;
use notification::{Notification, send_notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Notify {
            app_name,
            replaces_id,
            title,
            body,
            icon,
            timeout,
        } => {
            let notification = Notification::new(app_name, replaces_id, title, body, icon, timeout);
            send_notification(notification).await?;
        }
        Commands::ListIcons { set } => {
            handle_icon_listing(set);
        }
        Commands::Defaults { pomodoro } => {
            if pomodoro {
                let notification = Notification::new(
                    String::from("Pomodoro"),
                    0,
                    String::from("Time's up!"),
                    String::from("Take a short break."),
                    String::from("alarm-clock-elapsed"),
                    10000,
                );
                send_notification(notification).await?;
            } else {
                println!("No default action specified.");
            }
        }
    }

    Ok(())
}
