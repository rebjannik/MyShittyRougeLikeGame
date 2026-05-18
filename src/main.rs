pub mod map_gen;
pub mod models;
pub mod ui;

use crate::models::{AppAction, GameState, MainMenuState, MenuOption};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};
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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_state = GameState::MainMenu;
    let mut menu_state = MainMenuState::new();

    loop {
        terminal.draw(|frame| match current_state {
            GameState::MainMenu => ui::render_menu(frame, &menu_state),
            GameState::InGame => {},
            GameState::CharacterCreation => {},
            GameState::Settings => {},
            GameState::Saves => {},
            _ => {},
        })?;

        if !event::poll(Duration::from_millis(16))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match current_state {
            GameState::MainMenu => match handle_main_menu_input(key, &mut menu_state) {
                AppAction::Continue => {}
                AppAction::ChangeState(next_state) => current_state = next_state,
                AppAction::Quit => return Ok(()),
                _ => {}
            },
            GameState::InGame => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
                    current_state = GameState::MainMenu;
                }
            }
            GameState::CharacterCreation => {}
            GameState::Settings => {}
            GameState::Saves => {}
            _ => {},
        }
    }
}

fn handle_main_menu_input(key: KeyEvent, menu_state: &mut MainMenuState) -> AppAction {
    match key.code {
        KeyCode::Up | KeyCode::Down => {
            menu_state.toggle();
            AppAction::Continue
        }
        KeyCode::Enter => match menu_state.selected {
            MenuOption::StartGame => AppAction::ChangeState(GameState::InGame),
            MenuOption::Exit => AppAction::Quit,
            _ => AppAction::Continue,
        },
        KeyCode::Char('q') => AppAction::Quit,
        _ => AppAction::Continue,
    }
}
