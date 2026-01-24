use std::collections::HashMap;
use std::error::Error;

use clap::Parser;
use clap::Subcommand;
use zbus::{Connection, proxy, zvariant::Value};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Notify {
        /// Application name for the notification
        #[arg(short, long, default_value_t = String::from("my_app"))]
        app_name: String,

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
        /// List available icons
        #[arg(short, long, default_value_t = false)]
        list: bool,
    },
}

pub struct Notification {
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub icon: String,
    pub timeout: i32,
}

impl Notification {
    pub fn new(app_name: String, title: String, body: String, icon: String, timeout: i32) -> Self {
        Self {
            app_name,
            title,
            body,
            icon,
            timeout,
        }
    }
}

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_i32: u32,
        app_icons: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub async fn send_notification(notification: Notification) -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    let _reply = proxy
        .notify(
            &notification.app_name,
            0,
            &notification.icon,
            &notification.title,
            &notification.body,
            &[],
            HashMap::new(),
            notification.timeout,
        )
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Notify {
            app_name,
            title,
            body,
            icon,
            timeout,
        } => {
            let notification = Notification::new(app_name, title, body, icon, timeout);
            send_notification(notification).await?;
        }
        Commands::ListIcons { list: _ } => {
            todo!("Icon listing not implemented yet");
        }
    }

    Ok(())
}
