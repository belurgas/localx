pub mod flf_grub;
// pub mod time_compile;

use super::widgets::app::KeyboardEvent;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_event() -> color_eyre::Result<Option<KeyboardEvent>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

pub fn handle_key(key: event::KeyEvent) -> Option<KeyboardEvent> {
    match key.code {
        KeyCode::Up => Some(KeyboardEvent::Up),
        KeyCode::Down => Some(KeyboardEvent::Down),
        KeyCode::Enter => Some(KeyboardEvent::Enter),
        KeyCode::Esc => Some(KeyboardEvent::Out),
        KeyCode::Char('q') | KeyCode::Char('й') => Some(KeyboardEvent::Quit),
        _ => None,
    }
}
