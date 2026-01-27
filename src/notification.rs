use crate::actions::ACTIONS;
use std::collections::HashMap;
use std::error::Error;

use zbus::{Connection, proxy, zvariant::Value};

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
    #[allow(clippy::too_many_arguments)]
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

    #[zbus(signal)]
    pub async fn action_invoked(
        ctx: &SignalContext<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn notification_replied(
        ctx: &SignalContext<'_>,
        id: u32,
        text: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn notification_closed(
        ctx: &SignalContext<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()>;
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
