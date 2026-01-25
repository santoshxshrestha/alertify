use clap::Parser;
use std::collections::HashMap;
use std::error::Error;

use clap::ValueEnum;
use zbus::{Connection, proxy, zvariant::Value};

pub mod actions;
pub mod cli;
pub mod icons;
use actions::ACTIONS;
use cli::Cli;
use cli::Commands;

use crate::icons::{
    STD_ACTION_ICONS, STD_ANIMATION_ICONS, STD_APPLICATION_ICONS, STD_CATEGORY_ICONS,
    STD_DEVICE_ICONS, STD_EMBLEM_ICONS, STD_EMOTION_ICONS, STD_INTERNATIONAL_ICONS,
    STD_MIME_TYPE_ICONS, STD_PLACE_ICONS, STD_STATUS_ICONS,
};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum IconSet {
    All,
    Actions,
    Animations,
    Applications,
    Categories,
    Devices,
    Emblems,
    Emotes,
    International,
    MimeTypes,
    Places,
    Status,
}

pub struct Notification {
    pub app_name: String,
    pub replaces_id: u32,
    pub title: String,
    pub body: String,
    pub icon: String,
    pub timeout: i32,
}

impl Notification {
    pub fn new(
        app_name: String,
        replaces_id: u32,
        title: String,
        body: String,
        icon: String,
        timeout: i32,
    ) -> Self {
        Self {
            app_name,
            replaces_id,
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
        hints: HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub async fn send_notification(notification: Notification) -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    // FIXME: this is currently not working in my sys according to the [doc](https://specifications.freedesktop.org/notification/latest/hints.html)
    // it should play the "alarm-clock-elapsed" sound if available
    let value = Value::Str("message-new-instant".into());
    let mut hint = HashMap::new();
    hint.insert("sound-name", value);

    let reply = proxy
        .notify(
            &notification.app_name,
            notification.replaces_id,
            &notification.icon,
            &notification.title,
            &notification.body,
            &[
                ACTIONS[0].0,
                ACTIONS[0].1,
                ACTIONS[1].0,
                ACTIONS[1].1,
                ACTIONS[2].0,
                ACTIONS[2].1,
                ACTIONS[3].0,
                ACTIONS[3].1,
            ],
            hint,
            notification.timeout,
        )
        .await?;

    dbg!(reply);

    Ok(())
}

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
            let mut lists = Vec::new();

            match set {
                IconSet::All => {
                    lists.push(("actions", STD_ACTION_ICONS));
                    lists.push(("animations", STD_ANIMATION_ICONS));
                    lists.push(("applications", STD_APPLICATION_ICONS));
                    lists.push(("categories", STD_CATEGORY_ICONS));
                    lists.push(("devices", STD_DEVICE_ICONS));
                    lists.push(("emblems", STD_EMBLEM_ICONS));
                    lists.push(("emotes", STD_EMOTION_ICONS));
                    lists.push(("international", STD_INTERNATIONAL_ICONS));
                    lists.push(("mimetypes", STD_MIME_TYPE_ICONS));
                    lists.push(("places", STD_PLACE_ICONS));
                    lists.push(("status", STD_STATUS_ICONS));
                }
                IconSet::Actions => lists.push(("actions", STD_ACTION_ICONS)),
                IconSet::Animations => lists.push(("animations", STD_ANIMATION_ICONS)),
                IconSet::Applications => lists.push(("applications", STD_APPLICATION_ICONS)),
                IconSet::Categories => lists.push(("categories", STD_CATEGORY_ICONS)),
                IconSet::Devices => lists.push(("devices", STD_DEVICE_ICONS)),
                IconSet::Emblems => lists.push(("emblems", STD_EMBLEM_ICONS)),
                IconSet::Emotes => lists.push(("emotes", STD_EMOTION_ICONS)),
                IconSet::International => lists.push(("international", STD_INTERNATIONAL_ICONS)),
                IconSet::MimeTypes => lists.push(("mimetypes", STD_MIME_TYPE_ICONS)),
                IconSet::Places => lists.push(("places", STD_PLACE_ICONS)),
                IconSet::Status => lists.push(("status", STD_STATUS_ICONS)),
            }

            for (label, icons) in lists {
                println!("[{label}]");
                for (icon_name, description) in icons {
                    println!("{}: {}\n", icon_name, description);
                }
                println!();
            }
        }
    }

    Ok(())
}
