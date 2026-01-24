use std::collections::HashMap;
use std::error::Error;

use clap::Parser;
use zbus::{Connection, proxy, zvariant::Value};

#[derive(Parser)]
struct Cli {
    /// Application name
    #[arg(short, long, default_value_t = String::from("my_app"))]
    app_name: String,

    /// Notification title
    #[arg(short, long, default_value_t = String::from("A summary"))]
    title: String,

    /// Notification body
    #[arg(short, long, default_value_t = String::from("Some body"))]
    body: String,

    /// Icon name
    /// Default is "dialog-information"
    #[arg(short, long, default_value_t = String::from("dialog-information"))]
    icon: String,

    /// Notification timeout in milliseconds
    #[arg(short = 's', long, default_value_t = 5000)]
    timeout: i32,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let notification = Notification::new(cli.app_name, cli.title, cli.body, cli.icon, cli.timeout);

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
