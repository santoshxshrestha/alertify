use chrono;
use crossterm::cursor;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, KeyEvent};
use crossterm::terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

use std::io::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use crate::notification::{Notification, send_notification};
use std::error::Error;
use std::time::Duration;

struct RawModeGuard;
impl RawModeGuard {
    fn new() -> Result<Self, std::io::Error> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

#[derive(Clone, Debug)]
pub enum PomodoroState {
    Work,
    Pause,
    Break,
}

pub fn handle_state(state: Arc<RwLock<PomodoroState>>) -> Result<(), Box<dyn Error>> {
    let _raw_mode_guard = RawModeGuard::new()?;

    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: Press,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: Press,
                ..
            }) => {
                let mut state = state.write().expect("Failed to acquire write lock");
                *state = PomodoroState::Break;
                return Ok(());
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
                    }
                    PomodoroState::Pause => {
                        *state = PomodoroState::Work;
                    }
                    PomodoroState::Break => {}
                }
            }
            _ => {}
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
        handle_state(clone_state).unwrap();
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let state = {
            let state = state.read().expect("Failed to acquire read lock");
            state.clone()
        };

        crossterm::execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;

        match state {
            PomodoroState::Work => {
                remaining_time = remaining_time.saturating_sub(Duration::from_secs(1));
                print!(
                    "Working... Remaining time: {}",
                    format_mm_ss(remaining_time.as_secs() as i64)
                );

                if remaining_time.as_secs() == 0 {
                    println!();
                    return send_notification(notification).await;
                }
            }
            PomodoroState::Pause => {
                print!(
                    "Paused... Remaining time: {}",
                    format_mm_ss(remaining_time.as_secs() as i64)
                );
            }
            PomodoroState::Break => {
                println!();
                return Ok(());
            }
        }

        io::stdout().flush()?;
    }
}

fn format_mm_ss(total_seconds: i64) -> String {
    let d = chrono::Duration::seconds(total_seconds.max(0));
    let minutes = d.num_minutes();
    let seconds = (d - chrono::Duration::minutes(minutes)).num_seconds();
    format!("{:02}:{:02}", minutes, seconds)
}
