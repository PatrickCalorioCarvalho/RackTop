use ratatui::{
    Frame,
    widgets::{Paragraph},
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Style, Modifier, Color},
    text::{Span, Line},
};

use figlet_rs::FIGfont;
use crate::{metrics::Metrics, app::Tab};
use super::{tab_system, tab_processes, tab_docker};

fn generate_ascii(title: &str) -> Vec<Line<'static>> {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(title);

    if let Some(fig) = figure {
        fig.to_string()
            .lines()
            .map(|l| {
                Line::from(Span::styled(
                    l.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect()
    } else {
        vec![Line::from(Span::styled(
            title.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))]
    }
}

pub fn draw_layout(
    f: &mut Frame,
    metrics: &Metrics,
    title: &str,
    tab: Tab,
) {
    let size = f.size();
    let ascii_lines = generate_ascii(title);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(ascii_lines.len() as u16),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let title_widget = Paragraph::new(ascii_lines)
        .alignment(Alignment::Center);

    f.render_widget(title_widget, chunks[0]);

    match tab {
        Tab::System => tab_system::draw(f, chunks[1], metrics),
        Tab::Processes => tab_processes::draw(f, chunks[1]),
        Tab::Docker => tab_docker::draw(f, chunks[1]),
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::raw("← → trocar      "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" sair"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));

    f.render_widget(footer, chunks[2]);
}