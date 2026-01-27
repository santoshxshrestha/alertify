use crate::notification::{Notification, send_notification};
use std::error::Error;

pub async fn handle_pomodoro() -> Result<(), Box<dyn Error>> {
    let notification = Notification::new(
        String::from("Pomodoro"),
        0,
        String::from("Time's up!"),
        String::from("Take a short break."),
        String::from("alarm-clock-elapsed"),
        10000,
    );
    send_notification(notification).await
}
