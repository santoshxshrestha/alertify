use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, KeyEvent};

use std::io::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use crate::notification::{Notification, send_notification};
use std::error::Error;
use std::time::Duration;

pub enum PomodoroState {
    Work,
    Pause,
    Break,
}

pub fn handle_state(state: Arc<RwLock<PomodoroState>>) -> Result<(), Box<dyn Error>> {
    loop {
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
            }) => {
                let mut state = state.write().expect("Failed to acquire write lock");
                match *state {
                    PomodoroState::Work => {
                        *state = PomodoroState::Pause;
                        println!("Paused!");
                    }
                    PomodoroState::Pause => {
                        *state = PomodoroState::Work;
                        println!("Resumed!");
                    }
                    PomodoroState::Break => {
                        println!("In break, cannot pause or resume.");
                    }
                }
            }
            _ => {
                println!("Other key pressed...");
            }
        }
    }
}

pub async fn handle_pomodoro() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(RwLock::new(PomodoroState::Work));
    let mut remaining_time = Duration::from_secs(25 * 60); // 25 minutes
    let notification = Notification::new(
        String::from("Pomodoro"),
        0,
        String::from("Time's up!"),
        String::from("Take a short break."),
        String::from("dialog-information"),
        10000,
    );

    let clone_state = Arc::clone(&state);

    thread::spawn(move || {
        println!("we are in the thread");
        handle_state(clone_state).unwrap();
        println!("exiting thread");
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        match *state.read().expect("Failed to acquire read lock") {
            PomodoroState::Work => {
                remaining_time -= Duration::from_secs(1);
                println!("Working... Remaining time: {:?}", remaining_time);
                std::io::stdout().flush().unwrap();
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
