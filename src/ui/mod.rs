#[allow(unused)]
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::models::{MainMenuState, MenuOption};

pub fn render_menu(frame: &mut Frame, menu_state: &MainMenuState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Top padding
            Constraint::Length(8),  // Height of your exact ASCII block
            Constraint::Length(10), // Gap
            Constraint::Min(5),     // Options space
        ])
        .split(frame.area());

    let title_lines = vec![
        Line::from(Span::styled(
            r"  __  __         _____ _     _ _   _           _____                        _        _ _         ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r" |  \/  |       / ____| |   (_) | | |         |  __ \                      | | |    (_) |       ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r" | \  / |_   _ | (___ | |__  _| |_| |_ _   _  | |__) |___  _   _  __ _  ___  | |     _| | _____ ",
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r" | |\/| | | | | \___ \| '_ \| | __| __| | | | |  _  // _ \| | | |/ _` |/ _ \ | |    | | |/ / _ \",
            Style::default().fg(Color::LightRed),
        )),
        Line::from(Span::styled(
            r" | |  | | |_| | ____) | | | | | |_| |_| |_| | | | \ \ (_) | |_| | (_| |  __/ | |____| |   <  __/",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            r" |_|  |_|\__, ||_____/|_| |_|_|\__|\__|\__, | |_|  \_\___/ \__,_|\__, |\___| |______|_|_|\_\___| ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            r"          __/ |                         __/ |                     __/ |                          ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            r"         |___/                         |___/                     |___/                           ",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let title_paragraph = Paragraph::new(title_lines).alignment(Alignment::Center);
    frame.render_widget(title_paragraph, chunks[1]);

    // Set up text styles for selection highlighting
    let start_style = if menu_state.selected == MenuOption::StartGame {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let exit_style = if menu_state.selected == MenuOption::Exit {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    // Build text lines
    let start_text = if menu_state.selected == MenuOption::StartGame {
        ">> Start Game <<"
    } else {
        "  Start Game  "
    };
    let exit_text = if menu_state.selected == MenuOption::Exit {
        ">> Exit <<"
    } else {
        " Exit "
    };

    let menu_lines = vec![
        Line::from(Span::styled(start_text, start_style)),
        Line::from(""),
        Line::from(Span::styled(exit_text, exit_style)),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "▲/▼ Arrows to Navigate  •  Enter to Confirm",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let menu_paragraph = Paragraph::new(menu_lines).alignment(Alignment::Center);
    frame.render_widget(menu_paragraph, chunks[3]);
}
