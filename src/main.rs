pub mod map_gen;
pub mod models;
pub mod ui;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::models::{GameState, MainMenuState, MenuOption};

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Application state variables
    let mut current_state = GameState::MainMenu;
    let mut menu_state = MainMenuState::new();

    let mut map = None;

    loop {
        terminal.draw(|frame| {
            match current_state {
                GameState::MainMenu => ui::render_menu(frame, &menu_state),
                GameState::InGame => {}
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match current_state {
                    GameState::MainMenu => match key.code {
                        KeyCode::Up | KeyCode::Down => {
                            menu_state.toggle();
                        }
                        KeyCode::Enter => match menu_state.selected {
                            MenuOption::StartGame => {
                                // Generate map and shift states
                                map = Some(map_gen::generate_map(80, 40).unwrap());
                                current_state = GameState::InGame;
                            }
                            MenuOption::Exit => break, // Break the loop to exit
                        },
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    GameState::InGame => match key.code {
                        KeyCode::Char('q') => {
                            // Pressing Q in-game kicks you back to menu
                            current_state = GameState::MainMenu;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
