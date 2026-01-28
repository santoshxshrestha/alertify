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
    pub fn handle_state(&mut self) -> Result<(), Box<dyn Error>> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: Press,
                    ..
                }) => {
                    std::process::exit(0);
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    kind: Press,
                    ..
                }) => match self {
                    PomodoroState::Work => {
                        *self = PomodoroState::Pause;
                        println!("Paused!");
                    }
                    PomodoroState::Pause => {
                        *self = PomodoroState::Work;
                        println!("Resumed!");
                    }
                    PomodoroState::Break => {
                        println!("In break, cannot pause or resume.");
                    }
                },
                _ => {
                    println!("Other key pressed...");
                }
            }
        }
        Ok(())
    }
}

pub async fn handle_pomodoro() -> Result<(), Box<dyn Error>> {
    let mut state = PomodoroState::Work;
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
        state.handle_state()?;
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
                println!("Paused... Remaining time: {:?}", remaining_time);
            }
            PomodoroState::Break => {
                return Ok(());
            }
        }
    }
}
