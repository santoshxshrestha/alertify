use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::poll;
use crossterm::event::{self, KeyEvent};

use crate::notification::{Notification, send_notification};
use std::error::Error;
use std::time::Duration;

pub enum PomodoroState {
    Work,
    Pause,
    Break,
}
impl PomodoroState {
    pub fn handle_state(&self) -> Result<(), Box<dyn Error>> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: Press,
                    ..
                }) => {
                    return Ok(());
                }
                _ => {
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

pub async fn handle_pomodoro() -> Result<(), Box<dyn Error>> {
    let state = PomodoroState::Work;
    let mut remaining_time = Duration::from_secs(25 * 60); // 25 minutes
    let notification = Notification::new(
        String::from("Pomodoro"),
        0,
        String::from("Time's up!"),
        String::from("Take a short break."),
        String::from("dialog-information"),
        10000,
    );

    loop {
        match state {
            PomodoroState::Work => {
                tokio::time::sleep(Duration::from_secs(1)).await;
                remaining_time -= Duration::from_secs(1);
                dbg!(remaining_time);
                if remaining_time.as_secs() == 0 {
                    return send_notification(notification).await;
                }
            }
            PomodoroState::Pause => {
                todo!("implement pause functionality by the use of crossterm");
            }
            PomodoroState::Break => {
                return Ok(());
            }
        }
    }
}
