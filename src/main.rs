mod app;
use app::{App, CurrentScreen, Input_Mode};

mod ui;
use ui::ui;

use ratatui::backend::{self, Backend, CrosstermBackend};
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{execute, terminal};
use ratatui::Terminal;

use std::error::Error;
use std::fs;
use std::io;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    if !app.current_dir.exists() {
        fs::write(&app.current_dir, "").unwrap();
    }

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match app.input_mode {
                    app::Input_Mode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.input_mode = Input_Mode::Editing;
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                    app::Input_Mode::Editing => match key.code {
                        KeyCode::Char(value) => {
                            app.value_input.push(value);
                        }
                        KeyCode::Left => {
                            app.move_cursor_left();
                        }
                        KeyCode::Right => {
                            app.move_cursor_right();
                        }
                        KeyCode::Esc => {
                            app.input_mode = Input_Mode::Normal;
                        }
                        KeyCode::Backspace => {
                            app.value_input.pop();
                        }
                        _ => {}
                    },
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => return Ok(()),
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backed = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backed)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
