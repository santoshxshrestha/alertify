pub type ActionEntry = (&'static str, &'static str);

pub static ACTIONS: &[ActionEntry] = &[
    ("snooze", "Snooze Notification"),
    ("dismiss", "Dismiss Notification"),
    ("break", "Take a Break"),
    ("restart", "Restart Application"),
];
