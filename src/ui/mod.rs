use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::models::{MainMenuState, MenuOption};

pub fn render_menu(frame: &mut Frame, menu_state: &MainMenuState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .contraints([
            Constraint::Percentage(50),  // Top padding
            Constraint::Length(10),      // Men box height
            Constraint::Percentage(30),  // Bottom padding
        ])
        .split(frame.ares());

    let menu_are = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),  // Left padding
            Constraint::Length(40),      // Menu box width
            Constraint::Percentage(35),  // Right padding
        ])
        .split(chunks[1])[1];

    // Set up text styles for selection highlighting
    let start_style = if menu_state.selected == MenuOption::StartGame{
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let exit_style = if menu_state.selected == MenuOption::Exit {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)

    } else {
        Style::default().fg(Color::White)
    };

    // bUild text lines
    let start_text = if menu_state.selected == MenuOption::StartGame{ ">> Start Game <<" } else { "  Start Game  " };
    let exit_text = if menu_state.selected == MenuOption::Exit {">> Exit <<"} else {" Exit ";

    let text = vec![
        Line::from(Span::styled("MY SHITTY ROUGELIKE", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::frim(""),
        Line::from(Span::styled(start_text, start_style)),
        Line::from(Span::styled(exit_text, exit_style)),
        Line::from(""),
        Line::from(Span:styled("Use Up/Down to navigate, Enter to select", Style::default(),fg(Color::Gray))),
    ];

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(text)
        .block(menu_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, menu_area);
    
}