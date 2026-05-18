pub mod map_gen;
pub mod models;
pub mod ui;

use crate::models::{GameState, MainMenuState, MenuOption};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run_app(&mut terminal);

    restore_terminal()?;
    result
}

fn run_app( terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_state = GameState::MainMenu;
    let mut menu_state = MainMenuState::new();

    loop {
        terminal.draw(|frame| match current_state {
            GameState::MainMenu => ui::render_menu(frame, &menu_state),
            GameState::InGame => {}
            GameState::CharacterCreation => {}
            GameState::Settings => {}
            GameState::Saves => {}
        })?;

        if !event::poll(Duration::from_millis(16))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match current_state {
            GameState::MainMenu => {
                if let Some(next_state) = handle_main_menu_input(key, &mut menu_state) {
                    current_state = next_state;
                }
            }
            GameState::InGame => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
                    current_state = GameState::MainMenu;
                }
            }
            GameState::CharacterCreation => {}
            GameState::Settings => {}
            GameState::Saves => {}
        }
    }
}

fn handle_main_menu_input(
    key: KeyEvent,
    menu_state: &mut MainMenuState,
) -> Option<GameState> {
    match key.code {
        KeyCode::Up | KeyCode::Down => {
            menu_state.toggle();
            None
        }
        KeyCode::Enter => match menu_state.selected {
            MenuOption::StartGame => Some(GameState::InGame),
            MenuOption::Exit => Some(GameState::Saves), 
        },
        KeyCode::Char('q') => Some(GameState::Saves), 
        _ => None,
    }
}