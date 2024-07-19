use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, Input_Mode};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header_block = Block::bordered();
    let header_block_text = Paragraph::new("Create new task / note").block(header_block);
    f.render_widget(header_block_text, chunks[0]);

    let center_main_block = Block::default().title("Files").borders(Borders::ALL);
    let center_main_text = Paragraph::new(format!("Current File: {:?}", app.current_dir)).block(center_main_block);
    let center_main_area = centered_rect(60, 25, f.size());

    f.render_widget(center_main_text, center_main_area);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => match app.input_mode {
                Input_Mode::Normal => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
                Input_Mode::Editing => {
                    Span::styled("Insert Mode", Style::default().fg(Color::LightGreen))
                }
            },
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::DarkGray)),
        }.to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        { 
            match app.input_mode {
                Input_Mode::Normal => Span::styled("Not editing anything", Style::default().fg(Color::DarkGray)),
                Input_Mode::Editing => Span::styled(format!("Editing {:?}", app.current_dir.file_name().unwrap()), Style::default().fg(Color::LightGreen)),
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(Q) to quit / (E) to start new task",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => match app.input_mode {
                Input_Mode::Normal => Span::styled(
                    "(ESC) to exit editing mode / (I) to insert",
                    Style::default().fg(Color::Red),
                ),
                Input_Mode::Editing => Span::styled(
                    "(ESC) to cancel / (ENTER) to complete",
                    Style::default().fg(Color::Red),
                ),
            },
            CurrentScreen::Exiting => Span::styled(
                "(Q) to quit / (E) to start new task",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Editing = app.current_screen {
        f.render_widget(Clear, chunks[1]);

        let fg_color = match app.input_mode {
            Input_Mode::Normal => Color::default(),
            Input_Mode::Editing => Color::Black,
        };

        let bg_color = match app.input_mode {
            Input_Mode::Normal => Color::default(),
            Input_Mode::Editing => Color::Yellow,
        };
        
        let edit_block = Block::default().title("Edit").borders(Borders::ALL);
        let edit_area = centered_rect(60, 25, f.size());
        let edit_text = Paragraph::new(app.value_input.clone())
            .block(edit_block).fg(fg_color).bg(bg_color);

        f.render_widget(edit_text, edit_area)
    }

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size());
        let popup_block = Block::default()
            .title("Exiting")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to quit? (y/n)",
            Style::default().fg(Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
